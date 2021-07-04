use crate::game::graphics::kernel::Texture;
use crate::game::offsets;
use macros::game_class;

game_class!(RenderTargetManager, {
    size: 0x3D8,
    location: offsets::globals::RenderTargetManager,
    fields: {
        // specific ones i can name
        // offscreen renderer is used to render models for UI elements like the character window
        [0x1E0] offscreen_render_target_1: *mut Texture,
        [0x1E8] offscreen_render_target_2: *mut Texture,
        [0x1F0] offscreen_render_target_3: *mut Texture,
        [0x1F8] offscreen_render_target_4: *mut Texture,
        [0x200] offscreen_g_buffer: *mut Texture,
        [0x208] offscreen_depth_stencil: *mut Texture,

        // these are related to offscreen renderer due to their size
        [0x210] offscreen_render_target_unk1: *mut Texture,

        [0x218] offscreen_render_target_unk2: *mut Texture,
        [0x220] offscreen_render_target_unk3: *mut Texture,

        [0x228] resolution_width: u32,
        [0x22C] resolution_height: u32,
        [0x230] shadow_map_width: u32,
        [0x234] shadow_map_height: u32,
        [0x238] near_shadow_map_width: u32,
        [0x23C] near_shadow_map_height: u32,
        [0x240] far_shadow_map_width: u32,
        [0x244] far_shadow_map_height: u32,
        [0x248] unk_bool_1: bool,
    }
});
