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

    pub fn log(&self, tag: &str, msg: &str) -> anyhow::Result<()> {
        let s = format!("[{}] {}", tag, msg);

        if let Ok(logger) = self.logger.lock() {
            if let Some(logger) = *logger {
                let c_str = CString::new(s.as_str())?;
                logger(c_str.as_ptr());
            }
        }
        println!("{}", s);

        Ok(())
    }
}

#[macro_export]
macro_rules! log {
    ($tag:expr, $($arg:tt)*) => {
        if let Some(logger) = crate::log::Logger::get_mut() {
            let _ = logger.log($tag, &format!($($arg)*));
        }
    }
}