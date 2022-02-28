use crate::image_thunks::ImageThunks;

use windows_sys::Win32::System::Diagnostics::Debug::IMAGE_NT_HEADERS32;

pub(crate) fn import_state() -> (isize, *const IMAGE_NT_HEADERS32) {
    use windows_sys::Win32::System::{
        LibraryLoader::GetModuleHandleA, SystemServices::IMAGE_DOS_HEADER,
    };

    unsafe {
        let image_base = GetModuleHandleA(std::ptr::null());
        let dos_headers = image_base as *const IMAGE_DOS_HEADER;
        let nt_headers =
            (image_base + (*dos_headers).e_lfanew as isize) as *const IMAGE_NT_HEADERS32;

        (image_base, nt_headers)
    }
}

pub(crate) fn import_tables() -> impl Iterator<Item = (String, ImageThunks)> {
    use crate::delay_load_modules::delay_load_modules;
    use crate::import_modules::import_modules;

    let (image_base, nt_headers) = import_state();
    import_modules(image_base, nt_headers).chain(delay_load_modules(image_base, nt_headers))
}
