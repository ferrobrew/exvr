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

fn iat_hook() -> anyhow::Result<()> {
    use import_tables::import_tables;
    use std::collections::{HashMap, HashSet};

    let dlls: HashSet<&str> = HashSet::from_iter(
        bindings::FUNCTIONS_BY_NAME
            .into_iter()
            .map(|((dll, _), ..)| dll),
    );
    let functions_by_name: HashMap<_, _> = bindings::FUNCTIONS_BY_NAME.into_iter().collect();

    for (library_name, functions) in import_tables().filter(|(l, _)| dlls.contains(l.as_str())) {
        for (function_name, ptr_to_function) in functions {
            if let Some(address) = functions_by_name.get(&(&library_name, &function_name)) {
                safe_patch(ptr_to_function, *address);
            }
        }
    }

    Ok(())
}

// need to keep detours alive for them to not be disabled
static mut DETOURS: Option<Vec<detour::RawDetour>> = None;
fn jc2_direct_hook() -> anyhow::Result<()> {
    use std::collections::HashMap;
    const FUNCTION_TO_ADDRESS: [((&str, &str), usize); 5] = [
        (("dxgi.dll", "CreateDXGIFactory"), 0xC396F6),
        (("d3d9.dll", "D3DPERF_SetOptions"), 0xC396B4),
        (("d3d10.dll", "D3D10CompileShader"), 0xC396D5),
        (("d3dx10_42.dll", "D3DX10CreateDevice"), 0xC39747),
        (("d3dx10_42.dll", "D3DX10GetFeatureLevel1"), 0xC39737),
    ];

    let functions: HashMap<_, _> = bindings::FUNCTIONS_BY_NAME.into_iter().collect();
    let address_to_replacement: Vec<_> = FUNCTION_TO_ADDRESS
        .into_iter()
        .map(|(p, a)| (a as *const (), *functions.get(&p).unwrap()))
        .collect();

    let mut detours = vec![];
    for (address, replacement) in address_to_replacement {
        unsafe {
            let detour = detour::RawDetour::new(address, replacement)?;
            detour.enable()?;
            detours.push(detour);
        }
    }
    unsafe {
        DETOURS = Some(detours);
    }

    Ok(())
}

fn load_impl() -> anyhow::Result<()> {
    const USE_DIRECT_HOOK: bool = true;

    if USE_DIRECT_HOOK {
        jc2_direct_hook()
    } else {
        iat_hook()
    }
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
