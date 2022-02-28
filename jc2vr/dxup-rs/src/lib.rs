mod bindings;
mod delay_load_modules;
mod image_thunks;
mod import_modules;
mod import_tables;
mod util;

use std::{fs::File, io::Write, os::raw::c_void, path::PathBuf};

use anyhow::Context;
use windows_sys::Win32::Foundation::HINSTANCE;

static mut THIS_MODULE: HINSTANCE = 0;
fn this_module_path() -> PathBuf {
    use std::{ffi::OsString, os::windows::prelude::OsStringExt};
    use windows_sys::Win32::System::LibraryLoader::GetModuleFileNameW;

    let mut buf = [0u16; 1024];
    unsafe {
        GetModuleFileNameW(THIS_MODULE, buf.as_mut_ptr(), buf.len() as _);
    }

    PathBuf::from(OsString::from_wide(&buf))
}

fn safe_patch(patch_address: *mut *const (), new_address: *const ()) {
    use windows_sys::Win32::System::Memory::{
        VirtualProtect, PAGE_PROTECTION_FLAGS, PAGE_READWRITE,
    };
    let mut old: PAGE_PROTECTION_FLAGS = 0;
    let size = std::mem::size_of_val(&new_address) as usize;
    unsafe {
        VirtualProtect(patch_address as *const _, size, PAGE_READWRITE, &mut old);
        *patch_address = new_address;
        VirtualProtect(patch_address as *const _, size, old, &mut old);
    }
}

fn load_impl() -> anyhow::Result<()> {
    use import_tables::import_tables;
    use std::collections::{HashMap, HashSet};
    let dlls: HashSet<&str> =
        HashSet::from_iter(bindings::IAT_HOOKS.into_iter().map(|((dll, _), ..)| dll));
    let addresses: HashMap<_, _> = bindings::IAT_HOOKS.into_iter().collect();

    let mut log = {
        let log_path = this_module_path()
            .parent()
            .context("failed to get parent path")?
            .join("dxup-rs.log");

        File::create(log_path)?
    };

    for (library_name, functions) in import_tables().filter(|(l, _)| dlls.contains(l.as_str())) {
        writeln!(log, "{}", library_name)?;

        for (function_name, ptr_to_function) in functions {
            write!(log, "  {}: {:X?}", function_name, unsafe {
                *ptr_to_function
            })?;

            if let Some(address) = addresses.get(&(&library_name, &function_name)) {
                write!(log, " -> {:X?}", address)?;

                safe_patch(ptr_to_function, *address);
            }

            writeln!(log)?;
        }
    }

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
