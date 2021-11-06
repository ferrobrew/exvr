use bindings::Windows::Win32::Foundation::HINSTANCE;
use std::os::raw::c_void;

static mut THIS_MODULE: HINSTANCE = HINSTANCE::NULL;

#[no_mangle]
#[allow(non_snake_case)]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "system" fn DllMain(module: HINSTANCE, _reason: u32, _: *mut c_void) -> bool {
    if THIS_MODULE.is_null() {
        THIS_MODULE = module;
    }
    true
}
