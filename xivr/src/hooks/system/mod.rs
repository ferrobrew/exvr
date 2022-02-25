pub mod framework;

pub struct HookState {
    _framework: framework::HookState,
}

impl HookState {
    pub fn new() -> anyhow::Result<HookState> {
        Ok(HookState {
            _framework: unsafe { framework::install()? },
        })
    }
}
