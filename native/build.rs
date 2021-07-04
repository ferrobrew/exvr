use anyhow::{Error, Result};
use quote::{format_ident, quote};
use std::fs;
use std::path::Path;
use yaml_rust::YamlLoader;

fn main() -> Result<()> {
    let out_dir = std::env::var("OUT_DIR")?;
    let out_dir = Path::new(&out_dir);

    let data = fs::read_to_string("../external/FFXIVClientStructs/ida/data.yml")?;
    let data = &YamlLoader::load_from_str(&data)?[0];

    let transform_pair = |(a, n): (&yaml_rust::yaml::Yaml, &yaml_rust::yaml::Yaml)| {
        let (n, a) = (
            n.as_str()
                .unwrap()
                .replace("g_", "")
                .replace("Client::", "")
                .replace("::", "_"),
            a.as_i64().unwrap() as u64,
        );
        let (n, a) = (format_ident!("{}", n), a - 0x1_4000_0000);

        quote! { pub const #n: u64 = #a; }
    };

    let version = data["version"]
        .as_str()
        .ok_or(Error::msg("Invalid version"))?;
    let globals = data["globals"]
        .as_hash()
        .ok_or(Error::msg("Not a hash"))?
        .iter()
        .map(transform_pair);
    let functions = data["functions"]
        .as_hash()
        .ok_or(Error::msg("Not a hash"))?
        .iter()
        .map(transform_pair);

    let tokens = quote! {
        pub const VERSION: &'static str = #version;
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

    fs::write(out_dir.join("offsets.rs"), tokens.to_string())?;

    Ok(())
}
