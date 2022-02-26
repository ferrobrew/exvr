use glob::glob;

fn main() {
    let mut paths = vec![];

    for entry in glob("src-cpp/d3d10/*.cpp").expect("Failed to read glob pattern") {
        let path = entry.unwrap();
        paths.push(path);
    }

    cc::Build::new()
        .cpp(true)
        .files(paths)
        .flag("/std:c++17")
        .define("NOMINMAX", None)
        .define("_SILENCE_CXX17_ITERATOR_BASE_CLASS_DEPRECATION_WARNING", None)
        .flag("/wd4100") // unreferenced formal parameter
        .flag("/wd4459") // hides global declaration
        .flag("/wd4267") // possible loss of data
        .object("d3dcompiler.lib")
        .object("dxgi.lib")
        .compile("libdxup10");
}