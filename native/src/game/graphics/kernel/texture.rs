use bindings::Windows::Win32::Graphics::Direct3D11::{ID3D11ShaderResourceView1, ID3D11Texture2D1};
use macros::game_class;

#[repr(u32)]
#[allow(dead_code)]
pub enum TextureFormat {
    R8G8B8A8 = 5200,
    D24S8 = 16976, // depth 28 stencil 8, see MS texture formats on google if you really care :)
}

game_class!(Texture, {
    size: 0xA8,
    fields: {
        [0x38] width: u32,
        [0x3C] height: u32,
        // for 3d textures like the material tiling texture
        [0x40] depth: u32,
        [0x44] mip_level: u8,
        [0x45] unk_35: u8,
        [0x46] unk_36: u8,
        [0x47] unk_37: u8,
        [0x48] texture_format: TextureFormat,
        [0x4C] flags: u32,
        [0x50] texture: *mut ID3D11Texture2D1,
        [0x58] shader_resource_view: *mut ID3D11ShaderResourceView1,
    },
});
