#![feature(command_access)]

use anyhow::{Error, Result};
use std::fs;
use std::path::Path;

fn generate_offsets_file() -> anyhow::Result<String> {
    use quote::{format_ident, quote};
    use yaml_rust::YamlLoader;

    const RELATIVE_PATH: &'static str = "../external/FFXIVClientStructs/ida/data.yml";
    println!("cargo:rerun-if-changed={}", RELATIVE_PATH);

    let data = fs::read_to_string(RELATIVE_PATH)?;
    let data = &YamlLoader::load_from_str(&data)?[0];

    let transform_pair = |(a, n): (&yaml_rust::yaml::Yaml, &yaml_rust::yaml::Yaml)| {
        let (n, a) = (
            n.as_str()
                .unwrap()
                .replace("g_", "")
                .replace("Client::", "")
                .replace(".", "::")
                .replace("::", "_"),
            a.as_i64().unwrap() as u64,
        );
        let (n, a) = (format_ident!("{}", n), a - 0x1_4000_0000);

        quote! { pub const #n: u64 = #a; }
    };

    let version = data["version"]
        .as_str()
        .ok_or_else(|| Error::msg("Invalid version"))?;
    let globals = data["globals"]
        .as_hash()
        .ok_or_else(|| Error::msg("Not a hash"))?
        .iter()
        .map(transform_pair);
    let functions = data["functions"]
        .as_hash()
        .ok_or_else(|| Error::msg("Not a hash"))?
        .iter()
        .map(transform_pair);

    let tokens = quote! {
        pub const VERSION: &str = #version;
        #[allow(dead_code, non_upper_case_globals)]
        pub mod offsets {
            pub mod globals {
                #(#globals)*
            }

            pub mod functions {
                #(#functions)*
            }
        }
    };

    Ok(tokens.to_string())
}

fn compile_shaders(out_dir: &Path) -> anyhow::Result<()> {
    use normpath::PathExt;
    use registry::{Data, Hive, Security};
    use std::path::PathBuf;
    use std::process::Command;

    const RELATIVE_PATH: &'static str = "shaders/";
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

    fs::write(out_dir.join("offsets.rs"), generate_offsets_file()?)?;
    compile_shaders(out_dir)?;

    Ok(())
}
