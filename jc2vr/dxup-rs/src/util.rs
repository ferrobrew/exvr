use std::{ffi::CStr, os::raw::c_char};

pub(crate) fn cstr_ptr_to_string(s: *const c_char) -> String {
    unsafe { CStr::from_ptr(s).to_string_lossy().to_string() }
}
