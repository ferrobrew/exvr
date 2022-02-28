mod bindings;
mod delay_load_modules;
mod image_thunks;
mod import_modules;
mod util;

use std::{
    ffi::OsString,
    fs::File,
    io::Write,
    os::{raw::c_void, windows::prelude::OsStringExt},
    path::PathBuf,
};

use delay_load_modules::delay_load_modules;
use import_modules::import_modules;

use anyhow::Context;
use windows_sys::Win32::{
    Foundation::HINSTANCE,
    System::{
        Diagnostics::Debug::IMAGE_NT_HEADERS32,
        LibraryLoader::{GetModuleFileNameW, GetModuleHandleA},
        SystemServices::IMAGE_DOS_HEADER,
    },
};

static mut THIS_MODULE: HINSTANCE = 0;

fn load_impl() -> anyhow::Result<()> {
    let parent_path = unsafe {
        let mut buf = [0u16; 1024];
        GetModuleFileNameW(THIS_MODULE, buf.as_mut_ptr(), buf.len() as _);

        PathBuf::from(OsString::from_wide(&buf))
            .parent()
            .map(|p| p.to_path_buf())
            .context("failed to get parent path")?
    };

    let mut file = File::create(parent_path.join("dxup-rs.log"))?;

    const DLLS: &[&str] = &["dxgi.dll", "d3d9.dll", "d3d10.dll", "d3dx10_42.dll"];
    let (image_base, nt_headers) = unsafe {
        let image_base = GetModuleHandleA(std::ptr::null());
        let dos_headers = image_base as *const IMAGE_DOS_HEADER;
        let nt_headers =
            (image_base + (*dos_headers).e_lfanew as isize) as *const IMAGE_NT_HEADERS32;

        (image_base, nt_headers)
    };

    for (library_name, image_thunks) in import_modules(image_base, nt_headers)
        .chain(delay_load_modules(image_base, nt_headers))
        .filter(|(library_name, _)| DLLS.iter().any(|f| *f == library_name))
    {
        writeln!(file, " {}", library_name)?;
        for (function_name, ptr_to_function) in image_thunks {
            writeln!(file, "  {}: {:X?}", function_name, unsafe {
                *ptr_to_function
            })?;
        }
    }

    // IAT replacements
    // dxgi.dll:CreateDXGIFactory
    // d3d9.dll:D3DPERF_SetOptions
    // d3d10.dll:D3D10CompileShader
    // d3dx10_42.dll:D3DX10CreateDevice
    // d3dx10_42.dll:D3DX10GetFeatureLevel1

    Ok(())
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "system" fn load(_: *mut u64, _: *mut u64) {
    load_impl().unwrap();
}

#[no_mangle]
#[allow(non_snake_case)]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "system" fn DllMain(module: HINSTANCE, reason: u32, _: *mut c_void) -> bool {
    if reason == windows_sys::Win32::System::SystemServices::DLL_PROCESS_ATTACH {
        THIS_MODULE = module;
    }
    true
}
