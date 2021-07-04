use crate::game::offsets;
use macros::game_class;

use bindings::Windows::Win32::Foundation::HWND;

game_class!(Window, {
    fields: {
        [0x18] handle: HWND
    }
});

game_class!(Framework, {
    location: offsets::globals::Framework,
    fields: {
        [0x7A8] window: Window
    }
});
