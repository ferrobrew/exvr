use glob::glob;

fn main() -> anyhow::Result<()> {
    const DEPENDENCIES: &[&str] = &["dxgi", "d3d11", "d3dcompiler"];
    for dependency in DEPENDENCIES {
        println!("cargo:rustc-link-lib={}", dependency);
    }

    println!("cargo:rerun-if-changed=src-cpp");
    let mut build = cc::Build::new();
    build
        .cpp(true)
        // files
        .include("src-cpp/d3d10_1") // necessary for dxgi
        .files(
            std::iter::once("src-cpp/dxup_guids.cpp".into())
                .chain(glob("src-cpp/d3d10_1/*.cpp")?.filter_map(Result::ok))
                .chain(glob("src-cpp/dxgi/*.cpp")?.filter_map(Result::ok)),
        )
        // compiler flags
        .flag("/std:c++17")
        .flag("/wd4100") // unreferenced formal parameter
        .flag("/wd4459") // hides global declaration
        .flag("/wd4267") // possible loss of data
        .flag("/wd5205"); // delete of an abstract class

    let is_debug = std::env::var("PROFILE") == Ok("debug".to_string());
    let mut defines = vec![
        "DXUP_EXPORTS",
        "DXUP_DXGI_EXPORTS",
        "NOMINMAX",
        "_SILENCE_CXX17_ITERATOR_BASE_CLASS_DEPRECATION_WARNING",
    ];
    if is_debug {
        defines.push("_DEBUG");
    }
    for define in defines {
        build.define(define, None);
    }

    build.compile("libdxup");

    Ok(())
}
