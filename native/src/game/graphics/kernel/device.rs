use crate::game::graphics::kernel::SwapChain;
use crate::game::offsets;
use macros::game_class;

use bindings::Windows::Win32::Graphics::Direct3D11::{
    ID3D11Device, ID3D11DeviceContext, D3D_FEATURE_LEVEL,
};
use std::os::raw::c_void;

game_class!(Device, {
    size: 0x210,
    location: offsets::globals::KernelDevice,
    fields: {
        [0x10] immediate_context: *mut c_void,
        [0x80] swapchain: &'static mut SwapChain,
        [0x94] feature_level: D3D_FEATURE_LEVEL,
        [0xA8] device: &'static mut ID3D11Device,
        [0xB0] device_context: &'static mut ID3D11DeviceContext,
    },
});