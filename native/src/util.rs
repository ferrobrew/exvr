#[macro_export]
macro_rules! singleton {
    ($class_name:ty $(, $arg_name:ident : $arg_type:ty)*) => {
        static mut INSTANCE: ::once_cell::unsync::OnceCell<$class_name> =
            ::once_cell::unsync::OnceCell::new();

        impl $class_name {
            pub fn create($($arg_name : $arg_type),*) -> anyhow::Result<()> {
                unsafe {
                    INSTANCE
                        .set(<$class_name>::new( $($arg_name),* )?)
                        .map_err(|_| anyhow::Error::msg("failed to set command stream"))?
                };

                Ok(())
            }

            pub fn destroy() {
                if unsafe { INSTANCE.get() }.is_none() { return; }
                let _ = unsafe { INSTANCE.take().unwrap() };
            }

            #[allow(dead_code)]
            pub fn get() -> Option<&'static $class_name> {
                unsafe { INSTANCE.get() }
            }

            #[allow(dead_code)]
            pub fn get_mut() -> Option<&'static mut $class_name> {
                unsafe { INSTANCE.get_mut() }
            }
        }
    };
}

pub fn handle_error<T>(result: anyhow::Result<T>) -> Option<T> {
    match result {
        Ok(val) => Some(val),
        Err(e) => {
            log!("Top-level uncaught error: {:?} {:?}", e, e.backtrace());
            None
        }
    }
}

pub fn handle_error_in_block<T: std::default::Default, F>(block: F) -> T
    where F: Fn() -> anyhow::Result<T>
{
    handle_error(block()).unwrap_or(Default::default())
}