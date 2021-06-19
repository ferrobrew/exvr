#include "StereoRender.h"

#include "xr/XR.h"
#include "xr/Session.h"
#include "xr/SessionManager.h"
#include "xr/XRLinear.h"

namespace xr
{
void StereoRender::Init()
{
	int i = 1;
	for (auto& eye : m_Eyes)
		eye.Init(i--);
}

void StereoRender::Destroy()
{
	for (auto& eye : m_Eyes)
		eye.Destroy();
}

void StereoRender::Render(const std::function<void()>& fn)
{
	for (auto& eye : m_Eyes)
	{
		m_EyeIndex = eye.m_Index;

		eye.BeginUpdate();
		fn();
		eye.EndUpdate();
	}
}

void StereoRender::RenderEyes()
{
	const float segmentWidth = 640.0f;	// TODO

	for (auto& eye : m_Eyes)
		eye.Render(segmentWidth * eye.m_Index, 0, 640.0f, 720.0f);	// TODO
}

uint32_t StereoRender::GetImage(size_t index) const
{
	return m_Eyes[index].GetImage();
}

void Eye::Init(int index)
{
	m_Index = index;
}

void Eye::Destroy()
{
}

void Eye::BeginUpdate()
{
	ApplyMatrix();
	// Apply view matrix here
}

void Eye::ApplyMatrix()
{
	XrFovf fov;

	// Get latest HMD data
	if (auto session = xr::GetSession())
	{
		fov = session->GetEyeFOV(m_Index);
		auto viewMatrix = session->GetHMDWorldPose(m_Index);
	}
	else
	{
		// TODO: find some default state here
	}

	float nearClip = 0.01f;
	float farClip = 1000.0f;  // TODO: get value from game
	const float right = tanf(fov.angleRight);
	const float left = tanf(fov.angleLeft);
	const float up = tanf(fov.angleUp);
	const float down = tanf(fov.angleDown);
	const float width = (right - left) * 0.5f;
	const float height = (up - down) * 0.5f;

	// Copy to game matrix here
}

void Eye::EndUpdate()
{
}

void Eye::Render(float x, float y, float w, float h)
{
	// Render image for the eye to headset swapchain here
}

uint32_t Eye::GetImage() const
{
	// Return image handle to copy to
	return 0;
}
};	// namespace xr
