mod window;

pub use window::*;

use crate::game::offsets;
use crate::game::system::framework::Window;

use macros::game_class;

game_class!(Framework, {
    location: offsets::globals::Framework,
    fields: {
        [0x7A8] window: &'static mut self::Window,
    }
});