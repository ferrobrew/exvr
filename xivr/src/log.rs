use std::ffi::CString;
use std::os::raw::{c_char, c_void};
use std::sync::Mutex;

use crate::singleton;

pub type LogType = extern "system" fn(s: *const c_char) -> c_void;

pub struct Logger {
    logger: Mutex<Option<LogType>>,
}
singleton!(Logger, logger: Option<LogType>);

impl Logger {
    pub fn new(logger: Option<LogType>) -> anyhow::Result<Logger> {
        Ok(Logger {
            logger: Mutex::new(logger),
        })
    }

    pub fn log(&self, tag: &str, msg: &str) {
        let s = format!("[{}] {}", tag, msg);

        let c_str = CString::new(s.as_str()).unwrap();
        let logger = self.logger.lock().unwrap();
        if let Some(logger) = *logger {
            logger(c_str.as_ptr());
        }
        println!("{}", s);
    }
}

#[macro_export]
macro_rules! log {
    ($tag:expr, $($arg:tt)*) => {
        crate::log::Logger::get_mut().unwrap().log($tag, &format!($($arg)*))
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
