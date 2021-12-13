mod window;

pub use window::*;

use crate::game::offsets;
use crate::game::system::framework::Window;

use macros::game_class;

game_class!(Framework, {
    location: offsets::classes::system::framework::Framework::INSTANCES[0],
    fields: {
        [0x7A8] window: &'static mut self::Window,
    }
});
