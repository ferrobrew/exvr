#pragma once

#include "game/Util.h"

#include <d3d11.h>

namespace game::graphics::kernel
{
class SwapChain
{
public:
	OFFSET_PROPERTY(uint32_t, Width, 0x38);
	OFFSET_PROPERTY(uint32_t, Height, 0x3C);
	OFFSET_PROPERTY(void*, BackBuffer, 0x58);
	OFFSET_PROPERTY(void*, DepthStencil, 0x60);
	OFFSET_PROPERTY(IDXGISwapChain*, DXGISwapChain, 0x68);
};
}  // namespace game::graphics::kernel