use windows_sys::Win32::System::{
    Diagnostics::Debug::IMAGE_NT_HEADERS32, SystemServices::IMAGE_IMPORT_DESCRIPTOR,
};

use crate::image_thunks::{image_thunks, ImageThunks};

pub(crate) struct ImportModules {
    image_base: isize,
    import_descriptor: *mut IMAGE_IMPORT_DESCRIPTOR,
}

impl Iterator for ImportModules {
    type Item = (String, ImageThunks);

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let name_offset = (*self.import_descriptor).Name;
            if name_offset == 0 {
                return None;
            }

            let library_name =
                crate::util::cstr_ptr_to_string((self.image_base + name_offset as isize) as _);
            let name_thunk_offset = (*self.import_descriptor).Anonymous.OriginalFirstThunk;
            let address_thunk_offset = (*self.import_descriptor).FirstThunk;

            self.import_descriptor = self.import_descriptor.add(1);

            Some((
                library_name,
                image_thunks(self.image_base, name_thunk_offset, address_thunk_offset),
            ))
        }
    }
}

pub(crate) fn import_modules(
    image_base: isize,
    nt_headers: *const IMAGE_NT_HEADERS32,
) -> ImportModules {
    unsafe {
        use windows_sys::Win32::System::Diagnostics::Debug::{
            IMAGE_DATA_DIRECTORY, IMAGE_DIRECTORY_ENTRY_IMPORT,
        };

        let imports_directory: IMAGE_DATA_DIRECTORY =
            (*nt_headers).OptionalHeader.DataDirectory[IMAGE_DIRECTORY_ENTRY_IMPORT as usize];
        let import_descriptor = (image_base + imports_directory.VirtualAddress as isize)
            as *mut IMAGE_IMPORT_DESCRIPTOR;

        ImportModules {
            image_base,
            import_descriptor,
        }
    }
}
