use crate::hooks::Patcher;

pub mod game_main;

pub struct HookState {
    _game_main: game_main::HookState,
}

impl HookState {
    pub fn new(_patcher: &mut Patcher) -> Option<HookState> {
        Some(HookState {
            _game_main: unsafe { game_main::hook_update()? },
        })
    }
}

