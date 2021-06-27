#pragma once

#include "game/Util.h"
#include "game/Addresses.h"

#include "game/graphics/kernel/Texture.h"

namespace game::graphics::render
{
class RenderTargetManager
{
public:
	static RenderTargetManager* Get()
	{
		return game::DerefBaseRelativePtr<RenderTargetManager*>(game::offsets::globals::RenderTargetManager);
	}

	// specific ones i can name
	// offscreen renderer is used to render models for UI elements like the character window
	OFFSET_PROPERTY(kernel::Texture*, OffscreenRenderTarget_1, 0x1E0);
	OFFSET_PROPERTY(kernel::Texture*, OffscreenRenderTarget_2, 0x1E8);
	OFFSET_PROPERTY(kernel::Texture*, OffscreenRenderTarget_3, 0x1F0);
	OFFSET_PROPERTY(kernel::Texture*, OffscreenRenderTarget_4, 0x1F8);
	OFFSET_PROPERTY(kernel::Texture*, OffscreenGBuffer, 0x200);
	OFFSET_PROPERTY(kernel::Texture*, OffscreenDepthStencil, 0x208);

 	// these are related to offscreen renderer due to their size
	OFFSET_PROPERTY(kernel::Texture*, OffscreenRenderTarget_Unk1, 0x210);

	OFFSET_PROPERTY(kernel::Texture*, OffscreenRenderTarget_Unk2, 0x218);
	OFFSET_PROPERTY(kernel::Texture*, OffscreenRenderTarget_Unk3, 0x220);

	OFFSET_PROPERTY(uint32_t, Resolution_Width, 0x228);
	OFFSET_PROPERTY(uint32_t, Resolution_Height, 0x22C);
	OFFSET_PROPERTY(uint32_t, ShadowMap_Width, 0x230);
	OFFSET_PROPERTY(uint32_t, ShadowMap_Height, 0x234);
	OFFSET_PROPERTY(uint32_t, NearShadowMap_Width, 0x238);
	OFFSET_PROPERTY(uint32_t, NearShadowMap_Height, 0x23C);
	OFFSET_PROPERTY(uint32_t, FarShadowMap_Width, 0x240);
	OFFSET_PROPERTY(uint32_t, FarShadowMap_Height, 0x244);
	OFFSET_PROPERTY(bool, UnkBool_1, 0x248);
};
}  // namespace game::graphics::kernel