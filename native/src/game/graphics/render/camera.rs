use crate::game::math;
use macros::game_class;

game_class!(Camera, {
    size: 0x128,
    attributes: #[derive(Clone)],
    fields: {
        [0x10] view_matrix: math::Mat4,
        [0x50] projection_matrix: math::Mat4,
        [0x90] eye_position: math::Vec3A,
        [0xA0] context_index_maybe: u32,
        [0xAC] near_distance: f32,
        [0xB0] far_distance: f32,
        [0xC0] unknown_matrix: math::Mat4,
        [0x120] constant_buffer: *mut u8,
    }
});
