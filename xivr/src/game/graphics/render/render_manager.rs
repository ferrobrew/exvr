use crate::game::offsets;
use crate::game::graphics::render;

use macros::game_class;

game_class!(RenderManager, {
    size: 0x2D6BD,
    location: offsets::globals::RenderManager,
    fields: {
        [0xAD28] render_camera: *mut render::Camera,
    }
});