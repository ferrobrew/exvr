use std::ffi::CString;
use std::os::raw::{c_char, c_void};
use std::sync::Mutex;

use crate::singleton;

pub type LogType = extern "system" fn(s: *const c_char) -> c_void;

pub struct Logger {
    logger: Mutex<LogType>,
}
singleton!(Logger, logger: LogType);

impl Logger {
    pub fn new(logger: LogType) -> anyhow::Result<Logger> {
        Ok(Logger {
            logger: Mutex::new(logger),
        })
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
        crate::log::Logger::get_mut().unwrap().log(&format!($($arg)*))
    }
}

#[macro_export]
macro_rules! dlog {
    ($e:expr) => {
        match $e {
            tmp => {
                crate::log::Logger::get_mut().unwrap().log(&format!(
                    "{}: {:?}",
                    stringify!($e),
                    tmp
                ));
                tmp
            }
        }
    };
}
