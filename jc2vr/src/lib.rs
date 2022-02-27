use std::os::raw::c_void;
use windows::Win32::Foundation::HINSTANCE;

static mut THIS_MODULE: Option<HINSTANCE> = None;

#[no_mangle]
#[allow(non_snake_case)]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "system" fn DllMain(module: HINSTANCE, _reason: u32, _: *mut c_void) -> bool {
    if THIS_MODULE.is_none() {
        THIS_MODULE = Some(module);
    }
    true
}
