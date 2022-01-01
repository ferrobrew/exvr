use crate::game::graphics::kernel::Texture;
use crate::game::offsets;
use macros::game_class;

game_class!(RenderTargetManager, {
    size: 0x3D8,
    location: offsets::classes::graphics::render::RenderTargetManager::INSTANCES[0],
    fields: {
        [0x58] rendered_game: &'static Texture,

        // specific ones i can name
        // offscreen renderer is used to render models for UI elements like the character window
        [0x1E0] offscreen_render_target_1: &'static Texture,
        [0x1E8] offscreen_render_target_2: &'static Texture,
        [0x1F0] offscreen_render_target_3: &'static Texture,
        [0x1F8] offscreen_render_target_4: &'static Texture,
        [0x200] offscreen_g_buffer: &'static Texture,
        [0x208] offscreen_depth_stencil: &'static Texture,

        // these are related to offscreen renderer due to their size
        [0x210] offscreen_render_target_unk1: &'static Texture,

        [0x218] offscreen_render_target_unk2: &'static Texture,
        [0x220] offscreen_render_target_unk3: &'static Texture,

        [0x228] resolution_width: u32,
        [0x22C] resolution_height: u32,
        [0x230] shadow_map_width: u32,
        [0x234] shadow_map_height: u32,
        [0x238] near_shadow_map_width: u32,
        [0x23C] near_shadow_map_height: u32,
        [0x240] far_shadow_map_width: u32,
        [0x244] far_shadow_map_height: u32,
        [0x248] unk_bool_1: bool,

        [0x358] some_rgb8_buffer: &'static Texture,
    }
});
