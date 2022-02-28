use windows_sys::Win32::System::{
    Diagnostics::Debug::IMAGE_NT_HEADERS32, WindowsProgramming::IMAGE_DELAYLOAD_DESCRIPTOR,
};

use crate::image_thunks::{image_thunks, ImageThunks};

pub(crate) struct DelayLoadModules {
    image_base: isize,
    delay_load_descriptor: *mut IMAGE_DELAYLOAD_DESCRIPTOR,
}

impl Iterator for DelayLoadModules {
    type Item = (String, ImageThunks);

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let dll_name_rva = (*self.delay_load_descriptor).DllNameRVA;
            if dll_name_rva == 0 {
                return None;
            }

            let library_name =
                crate::util::cstr_ptr_to_string((self.image_base + dll_name_rva as isize) as _);
            let name_thunk_offset = (*self.delay_load_descriptor).ImportNameTableRVA;
            let address_thunk_offset = (*self.delay_load_descriptor).ImportAddressTableRVA;

            self.delay_load_descriptor = self.delay_load_descriptor.add(1);

            Some((
                library_name,
                image_thunks(self.image_base, name_thunk_offset, address_thunk_offset),
            ))
        }
    }
}

pub(crate) fn delay_load_modules(
    image_base: isize,
    nt_headers: *const IMAGE_NT_HEADERS32,
) -> DelayLoadModules {
    unsafe {
        use windows_sys::Win32::System::Diagnostics::Debug::{
            IMAGE_DATA_DIRECTORY, IMAGE_DIRECTORY_ENTRY_DELAY_IMPORT,
        };

        let imports_directory: IMAGE_DATA_DIRECTORY =
            (*nt_headers).OptionalHeader.DataDirectory[IMAGE_DIRECTORY_ENTRY_DELAY_IMPORT as usize];
        let delay_load_descriptor = (image_base + imports_directory.VirtualAddress as isize)
            as *mut IMAGE_DELAYLOAD_DESCRIPTOR;

        DelayLoadModules {
            image_base,
            delay_load_descriptor,
        }
    }
}
