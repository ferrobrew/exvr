#pragma once

#include <vector>
#include <d3d11.h>
#include <openxr/openxr.h>

using XrSwapchainImage = struct XrSwapchainImageD3D11KHR;

namespace xr
{
using ImageHandle = uint32_t;

class Swapchain
{
public:
	operator XrSwapchain() { return m_Handle; }

	Swapchain(XrSession session, uint32_t width, uint32_t height, uint64_t format, XrSwapchainUsageFlags flags);
	~Swapchain();

	void AcquireImage();
	void CopyImage(ImageHandle handle);
	void ReleaseImage();

private:
	XrSwapchain m_Handle{ 0 };
	uint32_t m_Width{ 0 };
	uint32_t m_Height{ 0 };
	size_t m_ImageIndex{ 0 };
	std::vector<XrSwapchainImage> m_Images;
};
}  // namespace xr
