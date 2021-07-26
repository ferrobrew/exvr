use macros::game_class;

use bindings::Windows::Win32::Graphics::Direct3D11::ID3D11DeviceContext;

game_class!(ImmediateContext, {
    size: 0xFA8,
    fields: {
        [0x18] device_context: ID3D11DeviceContext,
    },
});
