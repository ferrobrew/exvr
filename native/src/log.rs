use once_cell::sync::OnceCell;
use std::ffi::CString;
use std::os::raw::{c_char, c_void};
use std::sync::Mutex;

pub type LogType = extern "system" fn(s: *const c_char) -> c_void;

pub static LOGGER: OnceCell<Logger> = OnceCell::new();
pub struct Logger {
    logger: Mutex<LogType>,
}

impl Logger {
    pub fn new(logger: LogType) -> Logger {
        Logger {
            logger: Mutex::new(logger),
        }
    }

    pub fn initialize_instance(logger: LogType) -> Option<()> {
        LOGGER.set(Logger::new(logger)).ok()
    }

    pub fn log(&self, s: &str) {
        let s = CString::new(s).unwrap();
        let logger = self.logger.lock().unwrap();
        (*logger)(s.as_ptr());
    }
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        #[allow(unused_unsafe)]
        crate::log::LOGGER.get().unwrap().log(&format!($($arg)*))
    }
}
