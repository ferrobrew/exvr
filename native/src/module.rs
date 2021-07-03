use bindings::Windows::Win32::Foundation::{HINSTANCE, PWSTR};
use bindings::Windows::Win32::System::LibraryLoader::GetModuleFileNameW;
use bindings::Windows::Win32::System::ProcessStatus::{
    K32EnumProcessModules, K32GetModuleInformation, MODULEINFO,
};
use bindings::Windows::Win32::System::Threading::GetCurrentProcess;
use std::ffi::OsString;
use std::io;
use std::mem;
use std::os::windows::ffi::OsStringExt;
use std::path::Path;
use std::slice;

#[derive(Debug)]
pub struct Module {
    pub module: HINSTANCE,
    pub path: Option<String>,
    pub base: *mut u8,
    pub entry_point: *mut u8,
    pub image_size: u32,
}

impl Module {
    pub fn from_handle(module: &HINSTANCE) -> Module {
        let mut mod_info = unsafe { std::mem::zeroed() };
        unsafe {
            K32GetModuleInformation(
                GetCurrentProcess(),
                module,
                &mut mod_info,
                mem::size_of::<MODULEINFO>() as u32,
            );
        }
        Module {
            module: *module,
            path: {
                let mut buf = [0u16; 1024];
                let size = unsafe {
                    GetModuleFileNameW(module, PWSTR(buf.as_mut_ptr()), buf.len() as u32)
                } as usize;
                let os = OsString::from_wide(&buf[0..size]);
                os.into_string().ok()
            },
            base: mod_info.lpBaseOfDll as *mut u8,
            entry_point: mod_info.EntryPoint as *mut u8,
            image_size: mod_info.SizeOfImage,
        }
    }

    pub fn get_all() -> Vec<Module> {
        let process = unsafe { GetCurrentProcess() };
        let hinstance_size = mem::size_of::<HINSTANCE>() as u32;
        let mut temp = HINSTANCE::NULL;
        let mut needed = 0u32;
        unsafe {
            K32EnumProcessModules(process, &mut temp, hinstance_size, &mut needed);
        }
        let mut buf = vec![HINSTANCE::NULL; (needed / hinstance_size) as usize];
        unsafe {
            K32EnumProcessModules(
                process,
                buf.as_mut_ptr(),
                hinstance_size * (buf.len() as u32),
                &mut needed,
            );
        }
        buf.iter().map(Module::from_handle).collect()
    }

    pub fn as_bytes(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.base as *const u8, self.image_size as usize) }
    }

    pub fn scan(&self, pattern: &str) -> Option<*mut u8> {
        patternscan::scan_first_match(io::Cursor::new(self.as_bytes()), pattern)
            .ok()
            .flatten()
            .map(|o| unsafe { self.base.offset(o as isize) })
    }

    pub fn scan_for_relative_callsite(&self, pattern: &str) -> Option<*mut u8> {
        let p = self.scan(pattern)?;
        let call = unsafe { slice::from_raw_parts(p as *const u8, 5) };
        let offset = i32::from_ne_bytes(call[1..].try_into().ok()?) + 5;
        Some(unsafe { p.offset(offset as isize) })
    }

    pub fn scan_after_ptr(&self, base: *const u8, pattern: &str) -> Option<*mut u8> {
        let index = self.abs_to_rel_addr(base) as usize;
        let slice = &self.as_bytes()[index..];

        patternscan::scan_first_match(io::Cursor::new(slice), pattern)
            .ok()
            .flatten()
            .map(|o| self.rel_to_abs_addr((index + o) as isize))
    }

    pub fn filename(&self) -> Option<String> {
        Path::new(self.path.as_ref()?)
            .file_name()
            .and_then(|s| s.to_str())
            .map(|s| s.to_string())
    }

    pub fn abs_to_rel_addr(&self, p: *const u8) -> isize {
        unsafe { p.offset_from(self.base) }
    }

    pub fn rel_to_abs_addr(&self, offset: isize) -> *mut u8 {
        unsafe { self.base.offset(offset) }
    }
}
