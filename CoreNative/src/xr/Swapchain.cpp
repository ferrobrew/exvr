#include "xr/Swapchain.h"

#include "xr/XRUtils.h"
#include <openxr/openxr_platform.h>

#include "game/graphics/kernel/Device.h"

namespace xr
{
Swapchain::Swapchain(XrSession session, uint32_t width, uint32_t height, uint64_t format, XrSwapchainUsageFlags flags)
{
#ifdef XR_USE_GRAPHICS_API_OPENGL
	glGetError();
#endif

	XrSwapchainCreateInfo info{ XR_TYPE_SWAPCHAIN_CREATE_INFO };
	info.arraySize = 1;
	info.format = format;
	info.width = width;
	info.height = height;
	info.mipCount = 1;
	info.faceCount = 1;
	info.sampleCount = 1;
	info.createFlags = 0;
	info.usageFlags = flags;
	CHECK_XRCMD(xrCreateSwapchain(session, &info, &m_Handle));

	uint32_t size;
	CHECK_XRCMD(xrEnumerateSwapchainImages(m_Handle, 0, &size, nullptr));

	m_Images.resize(size, { XR_TYPE_SWAPCHAIN_IMAGE_D3D11_KHR });
	CHECK_XRCMD(xrEnumerateSwapchainImages(m_Handle, (uint32_t)m_Images.size(), &size, reinterpret_cast<XrSwapchainImageBaseHeader*>(m_Images.data())));

	m_Width = width;
	m_Height = height;
}

Swapchain::~Swapchain()
{
	CHECK_XRCMD(xrDestroySwapchain(m_Handle));
}

void Swapchain::AcquireImage()
{
	uint32_t image_index;
	XrSwapchainImageAcquireInfo acquire_info{ XR_TYPE_SWAPCHAIN_IMAGE_ACQUIRE_INFO };
	CHECK_XRCMD(xrAcquireSwapchainImage(m_Handle, &acquire_info, &image_index));
	m_ImageIndex = image_index;

	XrSwapchainImageWaitInfo wait_info{ XR_TYPE_SWAPCHAIN_IMAGE_WAIT_INFO };
	wait_info.timeout = XR_INFINITE_DURATION;
	CHECK_XRCMD(xrWaitSwapchainImage(m_Handle, &wait_info));
};

void Swapchain::CopyImage(ImageHandle handle)
{
	auto ctx = game::graphics::kernel::Device::Get()->D3DDeviceContext();
	ctx->CopyResource(m_Images[m_ImageIndex].texture, handle);
}

void Swapchain::ReleaseImage()
{
	XrSwapchainImageReleaseInfo release_info{ XR_TYPE_SWAPCHAIN_IMAGE_RELEASE_INFO };
	CHECK_XRCMD(xrReleaseSwapchainImage(m_Handle, &release_info));
};
};	// namespace xr
