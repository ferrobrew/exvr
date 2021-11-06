use glob::glob;

fn main() {
    let mut paths = vec![];

    for entry in glob("src-cpp/d3d10/*.cpp").expect("Failed to read glob pattern") {
        let path = entry.unwrap();
        paths.push(path);
    }

    for entry in glob("src-cpp/d3d11/*.cpp").expect("Failed to read glob pattern") {
        let path = entry.unwrap();
        paths.push(path);
    }

    cc::Build::new()
        .cpp(true)
        .files(paths)
        .flag("/std:c++17")
        .define("NOMINMAX", None)
        .compile("libdxup10");
}