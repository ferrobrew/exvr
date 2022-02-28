mod bindings;
mod delay_load_modules;
mod image_thunks;
mod import_modules;
mod import_tables;
mod util;

use std::os::raw::c_void;

use windows_sys::Win32::Foundation::HINSTANCE;

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

    for (library_name, functions) in import_tables().filter(|(l, _)| dlls.contains(l.as_str())) {
        for (function_name, ptr_to_function) in functions {
            if let Some(address) = addresses.get(&(&library_name, &function_name)) {
                safe_patch(ptr_to_function, *address);
            }
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
pub unsafe extern "system" fn DllMain(_: HINSTANCE, _: u32, _: *mut c_void) -> bool {
    true
}
