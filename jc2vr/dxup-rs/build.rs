use glob::glob;

fn main() -> anyhow::Result<()> {
    const DEPENDENCIES: &[&str] = &["dxgi", "d3d11", "d3dcompiler"];
    for dependency in DEPENDENCIES {
        println!("cargo:rustc-link-lib={}", dependency);
    }

    println!("cargo:rerun-if-changed=src-cpp");
    let paths: Vec<_> = std::iter::once("src-cpp/dxup_guids.cpp".into())
        .chain(glob("src-cpp/d3d10_1/*.cpp")?.filter_map(Result::ok))
        .chain(glob("src-cpp/dxgi/*.cpp")?.filter_map(Result::ok))
        .collect();

    cc::Build::new()
        .cpp(true)
        // files
        .include("src-cpp/d3d10_1") // necessary for dxgi
        .files(paths)
        // compiler flags
        .flag("/std:c++17")
        .define("DXUP_EXPORTS", None)
        .define("DXUP_DXGI_EXPORTS", None)
        .define("NOMINMAX", None)
        .define(
            "_SILENCE_CXX17_ITERATOR_BASE_CLASS_DEPRECATION_WARNING",
            None,
        )
        .flag("/wd4100") // unreferenced formal parameter
        .flag("/wd4459") // hides global declaration
        .flag("/wd4267") // possible loss of data
        .flag("/wd5205") // delete of an abstract class
        // linker
        .compile("libdxup");

    Ok(())
}
