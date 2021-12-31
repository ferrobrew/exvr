pub(crate) struct DebugState {
    debug_utils: openxr::raw::DebugUtilsEXT,
    debug_utils_messenger: openxr::sys::DebugUtilsMessengerEXT,
}

impl DebugState {
    pub fn new(entry: &openxr::Entry, instance: &openxr::Instance) -> anyhow::Result<DebugState> {
        let debug_utils = unsafe { openxr::raw::DebugUtilsEXT::load(entry, instance.as_raw())? };
        let mut debug_utils_messenger = openxr::sys::DebugUtilsMessengerEXT::NULL;

        unsafe {
            use openxr::sys as xrs;

            unsafe extern "system" fn user_callback(
                _message_severity: xrs::DebugUtilsMessageSeverityFlagsEXT,
                _message_types: xrs::DebugUtilsMessageTypeFlagsEXT,
                callback_data: *const xrs::DebugUtilsMessengerCallbackDataEXT,
                _: *mut std::ffi::c_void,
            ) -> xrs::Bool32 {
                use std::ffi::CStr;

                let cb = &*callback_data;
                log!(
                    "xr::debug",
                    "{} {}: {}",
                    CStr::from_ptr(cb.message_id).to_string_lossy(),
                    CStr::from_ptr(cb.function_name).to_string_lossy(),
                    CStr::from_ptr(cb.message).to_string_lossy()
                );

                xrs::Bool32::from_raw(0)
            }

            let create_info = xrs::DebugUtilsMessengerCreateInfoEXT {
                ty: xrs::DebugUtilsMessengerCreateInfoEXT::TYPE,
                next: std::ptr::null(),
                message_severities: xrs::DebugUtilsMessageSeverityFlagsEXT::VERBOSE
                    | xrs::DebugUtilsMessageSeverityFlagsEXT::WARNING
                    | xrs::DebugUtilsMessageSeverityFlagsEXT::ERROR
                    | xrs::DebugUtilsMessageSeverityFlagsEXT::INFO,
                message_types: xrs::DebugUtilsMessageTypeFlagsEXT::GENERAL
                    | xrs::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                    | xrs::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
                user_callback: Some(user_callback),
                user_data: std::ptr::null_mut(),
            };

            (debug_utils.create_debug_utils_messenger)(
                instance.as_raw(),
                &create_info,
                &mut debug_utils_messenger,
            );
        };

        Ok(DebugState {
            debug_utils,
            debug_utils_messenger,
        })
    }
}

impl Drop for DebugState {
    fn drop(&mut self) {
        unsafe {
            (self.debug_utils.destroy_debug_utils_messenger)(self.debug_utils_messenger);
        }
    }
}
