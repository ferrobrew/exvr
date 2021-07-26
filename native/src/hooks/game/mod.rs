pub mod game_main;

pub struct HookState {
    _game_main: game_main::HookState,
}

impl HookState {
    pub fn new() -> Option<HookState> {
        Some(HookState {
            _game_main: unsafe { game_main::install()? },
        })
    }
}

