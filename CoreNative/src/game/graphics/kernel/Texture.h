#pragma once

#include "game/Util.h"

namespace game::graphics::kernel
{
    enum class TextureFormat : uint32_t
    {
        R8G8B8A8 = 5200,
        D24S8 = 16976 // depth 28 stencil 8, see MS texture formats on google if you really care :)
    };

    class Texture
    {
    public:
        OFFSET_PROPERTY(uint32_t, Width, 0x38);
        OFFSET_PROPERTY(uint32_t, Height, 0x3C);
        // for 3d textures like the material tiling texture
        OFFSET_PROPERTY(uint32_t, Depth, 0x40); 
        OFFSET_PROPERTY(uint8_t, MipLevel, 0x44);
        OFFSET_PROPERTY(uint8_t, Unk_35, 0x45);
        OFFSET_PROPERTY(uint8_t, Unk_36, 0x46);
        OFFSET_PROPERTY(uint8_t, Unk_37, 0x47);
        OFFSET_PROPERTY(kernel::TextureFormat, TextureFormat, 0x48);
        OFFSET_PROPERTY(uint32_t, Flags, 0x4C);
        OFFSET_PROPERTY(class ID3D11Texture2D1*, D3D11Texture2D, 0x50);
        OFFSET_PROPERTY(class ID3D11ShaderResourceView1*, D3D11ShaderResourceView, 0x58);
    };
}