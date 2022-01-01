use anyhow::Result;
use quote::{format_ident, quote};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use yaml_rust::yaml;

fn yaml_to_hash_iter(yaml: &yaml::Yaml) -> impl Iterator<Item = (&yaml::Yaml, &yaml::Yaml)> {
    yaml.as_hash().expect("not a hash").iter()
}

fn i64_ea_to_offset(addr: i64) -> u64 {
    (addr as u64) - 0x1_4000_0000
}

fn addr_name_pair_parse((addr, name): (&yaml::Yaml, &yaml::Yaml)) -> (String, u64) {
    (
        name.as_str()
            .unwrap()
            .replace("g_", "")
            .replace("Client::", "")
            .replace(".", "::")
            .replace("::", "_"),
        i64_ea_to_offset(addr.as_i64().unwrap()),
    )
}

fn parsed_const_pair_to_tokens((name, addr): &(String, u64)) -> proc_macro2::TokenStream {
    let (name, addr) = (format_ident!("{}", name), *addr as usize);
    quote! { pub const #name: usize = #addr; }
}

fn yaml_to_constants_tokens(yaml: &yaml::Yaml) -> Vec<proc_macro2::TokenStream> {
    yaml_to_hash_iter(yaml)
        .map(addr_name_pair_parse)
        .map(|kv| parsed_const_pair_to_tokens(&kv))
        .collect()
}

struct Instance {
    ea: u64,
    is_pointer: bool,
}

enum Object {
    Namespace(String, HashMap<String, Object>),
    Class(String, Vec<(String, u64)>, Vec<Instance>),
}
fn generate_object_token_stream(obj: &Object) -> proc_macro2::TokenStream {
    match obj {
        Object::Namespace(name, hm) => {
            let obj_tokens: Vec<_> = hm
                .iter()
                .map(|(_, obj)| generate_object_token_stream(obj))
                .collect();

            let body_tokens = quote! { #(#obj_tokens)* };
            if name.is_empty() {
                body_tokens
            } else {
                let name = format_ident!("{}", name);
                quote! {
                    pub mod #name {
                        #body_tokens
                    }
                }
            }
        }
        Object::Class(name, funcs, instances) => {
            let name = format_ident!("{}", name);
            let funcs = funcs.iter().map(parsed_const_pair_to_tokens);

            // we do not currently support non-pointers
            let instances: Vec<_> = instances
                .iter()
                .filter_map(|i| i.is_pointer.then(|| i.ea))
                .collect();
            let instance_count = instances.len();

            quote! {
                pub mod #name {
                    pub const INSTANCES: [u64; #instance_count] = [#(#instances,)*];

                    pub mod funcs {
                        #(#funcs)*
                    }
                }
            }
        }
    }
}

fn generate_offsets_file(out_dir: &Path) -> anyhow::Result<()> {
    use yaml_rust::YamlLoader;

    const RELATIVE_PATH: &str = "external/FFXIVClientStructs/ida/data.yml";
    println!("cargo:rerun-if-changed={}", RELATIVE_PATH);

    let data = fs::read_to_string(RELATIVE_PATH)?;
    let data = &YamlLoader::load_from_str(&data)?[0];

    let version = data["version"].as_str().expect("version was not a string");
    let globals = yaml_to_constants_tokens(&data["globals"]);
    let functions = yaml_to_constants_tokens(&data["functions"]);

    let mut root_object = Object::Namespace("".to_string(), HashMap::new());
    for (fq_name, data) in yaml_to_hash_iter(&data["classes"]) {
        use convert_case::{Case, Casing};

        let fq_name = fq_name
            .as_str()
            .expect("not a string")
            .replace("Client::", "");

        // yolo we don't support templates
        if fq_name.contains('<') {
            continue;
        }

        let (class_name, rustified_segments) = {
            let segments: Vec<_> = fq_name.split("::").collect();
            let (class_name, fq_path_segments) = segments.split_last().unwrap();
            let rustified_segments: Vec<_> = fq_path_segments
                .iter()
                .map(|s| s.to_case(Case::Snake))
                .collect();

            (class_name.to_string(), rustified_segments)
        };

        let instances = match data["instances"].as_vec() {
            Some(v) => v
                .iter()
                .map(|y| Instance {
                    ea: i64_ea_to_offset(y["ea"].as_i64().unwrap()),
                    is_pointer: y["pointer"].as_bool().unwrap_or(true),
                })
                .collect(),
            None => vec![],
        };

        let funcs = match data["funcs"].as_hash() {
            Some(hm) => hm.iter().map(addr_name_pair_parse).collect(),
            None => vec![],
        };

        let mut namespace = &mut root_object;
        for segment in &rustified_segments {
            if let Object::Namespace(_, ref mut hm) = namespace {
                namespace = hm
                    .entry(segment.clone())
                    .or_insert_with(|| Object::Namespace(segment.clone(), HashMap::new()));
            }
        }
        if let Object::Namespace(_, ref mut hm) = namespace {
            hm.insert(
                class_name.clone(),
                Object::Class(class_name.clone(), funcs, instances),
            );
        }
    }
    let classes = generate_object_token_stream(&root_object);

    let tokens = quote! {
        pub const VERSION: &str = #version;
        #[allow(dead_code, non_upper_case_globals, non_snake_case)]
        pub mod offsets {
            pub mod globals {
                #(#globals)*
            }

            pub mod functions {
                #(#functions)*
            }

            pub mod classes {
                #classes
            }
        }
    };

    let output = tokens.to_string();
    Ok(fs::write(out_dir.join("offsets.rs"), output)?)
}

fn compile_shaders(out_dir: &Path) -> anyhow::Result<()> {
    use normpath::PathExt;
    use registry::{Data, Hive, Security};
    use std::path::PathBuf;
    use std::process::Command;

    const RELATIVE_PATH: &str = "assets/shaders/";
    println!("cargo:rerun-if-changed={}", RELATIVE_PATH);

    for path in fs::read_dir(RELATIVE_PATH)?.filter_map(|res| res.map(|e| e.path()).ok()) {
        let shader_type = path
            .file_stem()
            .and_then(std::ffi::OsStr::to_str)
            .and_then(|x| x.split('_').last())
            .unwrap();

        let profile = match shader_type {
            "vertex" => "vs_5_0",
            "pixel" => "ps_5_0",
            _ => panic!("unsupported shader target"),
        };

        let regkey = Hive::LocalMachine.open(
            r"SOFTWARE\Microsoft\Windows Kits\Installed Roots",
            Security::Read,
        )?;
        let mut kits_root_10: PathBuf = match regkey.value("KitsRoot10")? {
            Data::String(s) => s.to_string_lossy(),
            _ => panic!(),
        }
        .into();
        kits_root_10.push("bin");
        kits_root_10.push("10.0.19041.0"); // TODO: use an actual version
        kits_root_10.push("x86");
        kits_root_10.push("fxc.exe");

        let input_path = path.normalize()?.into_path_buf();
        let output_path = out_dir
            .join(Path::new(path.file_stem().unwrap()).with_extension("dxbc"))
            .normalize_virtually()?
            .into_path_buf();

        let mut command = Command::new(kits_root_10);
        command.args([
            "/T",
            profile,
            "/E",
            "main",
            "/Fo",
            output_path.to_str().unwrap(),
            input_path.to_str().unwrap(),
        ]);
        let status = command.status()?;

        assert!(status.success());
    }

    Ok(())
}

fn main() -> Result<()> {
    let out_dir = std::env::var("OUT_DIR")?;
    let out_dir = Path::new(&out_dir);

    generate_offsets_file(out_dir)?;
    compile_shaders(out_dir)?;

    Ok(())
}
