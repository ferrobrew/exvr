use crate::game::graphics::kernel::{Context, ImmediateContext, SwapChain};
use crate::game::offsets;
use macros::game_class;

use windows::Win32::Graphics::Direct3D11::{
    ID3D11Device, ID3D11DeviceContext, D3D_FEATURE_LEVEL,
};

game_class!(Device, {
    size: 0x210,
    location: offsets::globals::KernelDevice,
    fields: {
        [0x8] contexts: *mut Context,
        [0x10] immediate_context: &'static mut ImmediateContext,
        [0x78] context_count: u32,
        [0x80] swapchain: &'static mut SwapChain,
        [0x94] feature_level: D3D_FEATURE_LEVEL,
        [0xA8] device: ID3D11Device,
        [0xB0] device_context: ID3D11DeviceContext,
    },
});
