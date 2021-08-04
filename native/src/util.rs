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
