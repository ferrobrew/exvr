use windows_sys::Win32::System::WindowsProgramming::IMAGE_THUNK_DATA32;

pub(crate) struct ImageThunks {
    image_base: isize,
    name_thunk: *const IMAGE_THUNK_DATA32,
    address_thunk: *mut IMAGE_THUNK_DATA32,
}

impl Iterator for ImageThunks {
    type Item = (String, *mut *const ());

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let address_of_data = (*self.name_thunk).u1.AddressOfData;
            if address_of_data == 0 {
                return None;
            }

            let function_name = crate::util::cstr_ptr_to_string(
                (self.image_base + address_of_data as isize + 2) as _,
            );
            let function = &mut (*self.address_thunk).u1.Function as *mut _ as *mut *const ();

            self.name_thunk = self.name_thunk.add(1);
            self.address_thunk = self.address_thunk.add(1);

            Some((function_name, function))
        }
    }
}

pub(crate) fn image_thunks(
    image_base: isize,
    name_thunk_offset: u32,
    address_thunk_offset: u32,
) -> ImageThunks {
    ImageThunks {
        image_base,
        name_thunk: (image_base + name_thunk_offset as isize) as _,
        address_thunk: (image_base + address_thunk_offset as isize) as _,
    }
}
