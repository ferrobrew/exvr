pub mod patcher;
pub use patcher::*;

pub mod game;
pub mod graphics;

use crate::singleton;

pub struct HookState {
    _game: game::HookState,
    _graphics: graphics::HookState,
}

impl HookState {
    pub fn new() -> anyhow::Result<HookState> {
        Ok(HookState {
            _game: game::HookState::new()?,
            _graphics: graphics::HookState::new()?,
        })
    }
}
singleton!(HookState);
