extern crate bindgen;

use std::env;
use std::path::{PathBuf, Path};

use cc::windows_registry as wr;

fn generate_lib_file(out_path: &Path) {
    let target = env::var("TARGET").unwrap();
    let cimgui_dll_path = "../../external/ImGui.NET/src/ImGui.NET-472/runtimes/win-x64/native/cimgui.dll";

    let mut dumpbin_exe = wr::find(&target, "dumpbin.exe").unwrap();
    let mut lib_exe = wr::find(&target, "lib.exe").unwrap();

    // Dump the exports, and create a def file.
    let exports = dumpbin_exe
        .args(["/exports", cimgui_dll_path])
        .output()
        .unwrap();
    let exports_out = String::from_utf8(exports.stdout).unwrap();
    let index_begin = exports_out.as_str().find("          1    0").unwrap();
    let index_end = exports_out.as_str().find("  Summary").unwrap();
    let def = &exports_out[index_begin..index_end];
    let def = def
        .lines()
        .filter(|l| l.len() > 0)
        .map(|l| (&l[l.rfind(' ').unwrap()..]).trim())
        .collect::<Vec<_>>()
        .join("\n");
    let def = "EXPORTS\n".to_string() + &def;
    let def_path = out_path.join("cimgui.def");
    std::fs::write(&def_path, def).unwrap();

    // Generate the lib
    {
        let dir = env::current_dir().unwrap();
        env::set_current_dir(&out_path).unwrap();
        lib_exe
            .args(["/def:cimgui.def", "/lib:cimgui.lib"])
            .output()
            .unwrap();
        env::set_current_dir(&dir).unwrap();
    }
    println!("cargo:rustc-link-search={}", out_path.to_string_lossy());
    println!("cargo:rustc-link-lib=cimgui");
}

fn generate_bindings(out_path: &Path) {
    println!("cargo:rerun-if-changed=../../external/cimgui/cimgui.h");

    let bindings = bindgen::Builder::default()
        .header("../../external/cimgui/cimgui.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .clang_arg("--language=c++")
        .clang_arg("-DCIMGUI_DEFINE_ENUMS_AND_STRUCTS")
        .prepend_enum_name(false)
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    generate_lib_file(&out_path);
    generate_bindings(&out_path);
}