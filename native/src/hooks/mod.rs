pub mod patcher;
pub use patcher::*;

pub mod game;
pub mod graphics;

pub struct HookState {
    _game: game::HookState,
    _graphics: graphics::HookState,
}

impl HookState {
    pub fn new(patcher: &mut Patcher) -> Option<HookState> {
        Some(HookState {
            _game: game::HookState::new(patcher)?,
            _graphics: graphics::HookState::new(patcher)?,
        })
    }
}
