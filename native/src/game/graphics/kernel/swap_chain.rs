use macros::game_class;
use crate::game::graphics::kernel::Texture;

use bindings::Windows::Win32::Graphics::Dxgi::IDXGISwapChain;

game_class!(SwapChain, {
    size: 0x70,
    fields: {
        [0x38] width: u32,
        [0x3C] height: u32,
        [0x58] back_buffer: *mut Texture,
        [0x60] depth_stencil: *mut Texture,
        [0x68] dxgi_swap_chain: *mut IDXGISwapChain,
    }
});