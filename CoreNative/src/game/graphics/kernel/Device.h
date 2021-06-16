#pragma once

#include "game/Util.h"
#include "game/Addresses.h"

#include <d3d11.h>

namespace game::graphics::kernel
{
class Device
{
public:
	static Device* Get()
	{
		return game::DerefBaseRelativePtr<Device*>(game::offsets::globals::KernelDevice);
	}

	OFFSET_PROPERTY(void*, ImmediateContext, 0x10);
	OFFSET_PROPERTY(class SwapChain*, SwapChain, 0x80);
	OFFSET_PROPERTY(D3D_FEATURE_LEVEL, FeatureLevel, 0x94);
	OFFSET_PROPERTY(ID3D11Device*, D3DDevice, 0xA8);
	OFFSET_PROPERTY(ID3D11DeviceContext*, D3DDeviceContext, 0xB0);
};
}  // namespace game::graphics::kernel