pub fn handle_error<T>(result: anyhow::Result<T>) -> Option<T> {
    match result {
        Ok(val) => Some(val),
        Err(e) => {
            log!("error", "Top-level uncaught error: {:?}", e);
            None
        }
    }
}

pub fn handle_error_in_block<T: std::default::Default, F>(block: F) -> T
where
    F: Fn() -> anyhow::Result<T>,
{
    handle_error(block()).unwrap_or_default()
}
