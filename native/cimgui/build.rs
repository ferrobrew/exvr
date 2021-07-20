extern crate bindgen;

use std::env;
use std::path::PathBuf;

use cc::windows_registry as wr;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let target = env::var("TARGET").unwrap();
    let cimgui_dll_path = "../../external/ImGui.NET/deps/cimgui/win-x64/cimgui.dll";

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
    // Tell cargo to tell rustc to link the compiled lib.
    println!("cargo:rustc-link-search={}", out_path.to_string_lossy());
    println!("cargo:rustc-link-lib=cimgui");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=../../external/cimgui/cimgui.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("../../external/cimgui/cimgui.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .clang_arg("--language=c++")
        .clang_arg("-DCIMGUI_DEFINE_ENUMS_AND_STRUCTS")
        // .clang_arg("-DCIMGUI_NO_EXPORT")
        .prepend_enum_name(false)
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
