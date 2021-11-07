use crate::singleton;

use std::os::raw::c_void;
use std::ptr;
use std::slice;

use windows::Win32::System::Memory::PAGE_PROTECTION_FLAGS;
use windows::Win32::System::Memory::{VirtualProtect, PAGE_EXECUTE_READWRITE};

struct Patch {
    address: *mut u8,
    original_bytes: Vec<u8>,
}

pub struct Patcher {
    patches: Vec<Patch>,
}
singleton!(Patcher);

impl Patcher {
    pub fn new() -> anyhow::Result<Patcher> {
        Ok(Patcher { patches: vec![] })
    }

    pub unsafe fn safe_write(&self, addr_ptr: *mut u8, bytes: &[u8]) {
        let mut old = PAGE_PROTECTION_FLAGS(0);
        VirtualProtect(
            addr_ptr as *mut c_void,
            bytes.len(),
            PAGE_EXECUTE_READWRITE,
            &mut old,
        );
        ptr::copy(bytes.as_ptr(), addr_ptr, bytes.len());
        VirtualProtect(addr_ptr as *mut c_void, bytes.len(), old, &mut old);
    }

    pub unsafe fn patch(&mut self, address: *mut u8, bytes: &[u8]) -> *mut u8 {
        self.patches.push(Patch {
            address,
            original_bytes: slice::from_raw_parts(address, bytes.len()).to_vec(),
        });

        self.safe_write(address, bytes);
        address
    }

    pub unsafe fn unpatch(&mut self, address: *mut u8) {
        if let Some(index) = self.patches.iter().position(|p| p.address == address) {
            {
                let patch = &self.patches[index];
                self.safe_write(patch.address, &patch.original_bytes);
            }
            self.patches.remove(index);
        }
    }
}

impl Drop for Patcher {
    fn drop(&mut self) {
        for patch in self.patches.iter().rev() {
            unsafe {
                self.safe_write(patch.address, &patch.original_bytes);
            }
        }
    }
}
