pub mod patcher;
pub use patcher::*;

pub mod graphics;
pub mod system;

use crate::singleton;

pub struct HookState {
    _system: system::HookState,
    _graphics: graphics::HookState,
}

impl HookState {
    pub fn new() -> anyhow::Result<HookState> {
        Ok(HookState {
            _system: system::HookState::new()?,
            _graphics: graphics::HookState::new()?,
        })
    }
}
singleton!(HookState);
