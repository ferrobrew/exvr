use std::ffi::OsString;
use std::io;
use std::mem;
use std::os::windows::ffi::OsStringExt;
use std::path::Path;
use std::slice;

use once_cell::unsync::OnceCell;

use bindings::Windows::Win32::Foundation::{HINSTANCE, PWSTR};
use bindings::Windows::Win32::System::LibraryLoader::GetModuleFileNameW;
use bindings::Windows::Win32::System::ProcessStatus::{
    K32EnumProcessModules, K32GetModuleInformation, MODULEINFO,
};
use bindings::Windows::Win32::System::Threading::GetCurrentProcess;

#[derive(Debug, Clone)]
pub struct Module {
    module: HINSTANCE,
    pub path: Option<String>,
    pub base: *mut u8,
    entry_point: *mut u8,
    image_size: u32,
    image_backup: Vec<u8>,
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
            image_backup: vec![],
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

    pub fn as_bytes_from_memory(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.base as *const u8, self.image_size as usize) }
    }

    pub fn backup_image(&mut self) {
        self.image_backup = self.as_bytes_from_memory().to_vec();
    }

    pub fn as_bytes(&self) -> &[u8] {
        if self.image_backup.is_empty() {
            self.as_bytes_from_memory()
        } else {
            &self.image_backup
        }
    }

    pub fn scan(&self, pattern: &str) -> anyhow::Result<*mut u8> {
        Ok(
            patternscan::scan_first_match(io::Cursor::new(self.as_bytes()), pattern)
                .transpose()
                .ok_or_else(|| anyhow::Error::msg("failed to scan"))?
                .map(|o| unsafe { self.base.add(o) })?,
        )
    }

    pub fn scan_for_relative_callsite(&self, pattern: &str) -> anyhow::Result<*mut u8> {
        let p = self.scan(pattern)?;
        let call = unsafe { slice::from_raw_parts(p as *const u8, 5) };
        let offset = i32::from_ne_bytes(call[1..].try_into()?) + 5;
        Ok(unsafe { p.offset(offset as isize) })
    }

    pub fn scan_after_ptr(&self, base: *const u8, pattern: &str) -> anyhow::Result<*mut u8> {
        let index = self.abs_to_rel_addr(base) as usize;
        let slice = &self.as_bytes()[index..];

        Ok(
            patternscan::scan_first_match(io::Cursor::new(slice), pattern)
                .transpose()
                .ok_or_else(|| anyhow::Error::msg("failed to scan"))?
                .map(|o| self.rel_to_abs_addr((index + o) as isize))?,
        )
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

pub static mut GAME_MODULE: OnceCell<Module> = OnceCell::new();
