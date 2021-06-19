#include "xr/Session.h"

#include "xr/InputManager.h"
#include "xr/XRUtils.h"
#include "xr/XRLinear.h"

#include <vector>
#include <stdio.h>

#include "game/graphics/kernel/Device.h"
#include "game/system/framework/Framework.h"

#include "Log.h"

#include <openxr/openxr_platform.h>

namespace xr
{
Session::Session(XrInstance instance, XrSystemId systemId)
{
	Initialize(instance, systemId);
	SetBlendMode(instance, systemId);
	SetDefaultViewConfiguration(instance, systemId);
	CreateSwapchains();
	CreateSpaces();
}

Session::~Session()
{
	m_Swapchains.clear();
	CHECK_XRCMD(xrDestroySpace(m_ViewSpace));
	CHECK_XRCMD(xrDestroySpace(m_OriginSpaces[xr::TrackingOrigin::EyeLevel]));
	CHECK_XRCMD(xrDestroySpace(m_TrackingSpaces[xr::TrackingOrigin::EyeLevel]));
	CHECK_XRCMD(xrDestroySpace(m_OriginSpaces[xr::TrackingOrigin::FloorLevel]));
	CHECK_XRCMD(xrDestroySpace(m_TrackingSpaces[xr::TrackingOrigin::FloorLevel]));
	CHECK_XRCMD(xrDestroySession(m_Handle));
}

const bool Session::HasViewConfigurationChanged() const
{
	return m_ViewConfigurationChanged;
}

const XrViewConfigurationView& Session::GetViewConfiguration() const
{
	return m_ViewConfiguration;
}

const XrView& Session::GetEyeView(uint32_t eye) const
{
	assert(eye < m_StereoViewCount);
	return m_Views[eye];
}

const uint32_t Session::GetEyeWidth() const
{
	return m_ViewConfiguration.recommendedImageRectWidth;
}

const uint32_t Session::GetEyeHeight() const
{
	return m_ViewConfiguration.recommendedImageRectHeight;
}

const XrFovf Session::GetEyeFOV(size_t index) const
{
	return m_ProjectionView[index].fov;
}

InputManager* Session::GetInputManager() const
{
	return m_InputManager.get();
}

bool Session::GetPose(XrPosef& pose, XrSpace space) const
{
	return GetPose(pose, space, &m_TrackingSpaces[0], m_TrackingOrigin);
}

bool Session::GetPose(XrPosef& pose, XrSpace space, const XrSpace* spaces, TrackingOrigin origin) const
{
	XrSpaceLocation location = { XR_TYPE_SPACE_LOCATION };
	CHECK_XRCMD(xrLocateSpace(space, spaces[origin], m_FrameState.predictedDisplayTime, &location));
	bool success = (location.locationFlags & (XR_SPACE_LOCATION_ORIENTATION_VALID_BIT | XR_SPACE_LOCATION_POSITION_VALID_BIT));

	if (success)
	{
		pose = location.pose;
	}

	return success;
}

void Session::Begin()
{
	assert(m_Handle != XR_NULL_HANDLE);
	XrSessionBeginInfo sessionBeginInfo{ XR_TYPE_SESSION_BEGIN_INFO };
	sessionBeginInfo.primaryViewConfigurationType = m_PrimaryViewConfiguration;
	CHECK_XRCMD(xrBeginSession(m_Handle, &sessionBeginInfo));
	m_Active = true;
}

void Session::End()
{
	assert(m_Handle != XR_NULL_HANDLE);
	CHECK_XRCMD(xrEndSession(m_Handle));
	m_Active = false;
}

void Session::BeginTick()
{
	assert(m_Handle != XR_NULL_HANDLE);
	XrFrameWaitInfo frame_wait_info{ XR_TYPE_FRAME_WAIT_INFO };
	CHECK_XRCMD(xrWaitFrame(m_Handle, &frame_wait_info, &m_FrameState));
}

static bool wasDown = false;
void Session::BeginFrame()
{
	assert(m_Handle != XR_NULL_HANDLE);
	assert(m_InputManager != nullptr);

	m_InputManager->Update(*this);

	// Hack to allow resetting view
	bool isDown = GetKeyState(VK_F5) & 0x800;

	if (isDown && !wasDown)
	{
		RecenterSpace();
	}

	wasDown = isDown;

	// Kick off any pending recenters
	if (m_ViewRecenter && m_ViewRecenterTime <= m_FrameState.predictedDisplayTime)
	{
		RecenterSpace(m_TrackingOrigin);
	}

	// Reset view configuration changed state
	if (m_ViewConfigurationChanged)
	{
		m_ViewConfigurationChanged = false;
	}

	const XrRect2Di image_rect = {
		{ 0, 0 }, { int32_t(m_ViewConfiguration.recommendedImageRectWidth), int32_t(m_ViewConfiguration.recommendedImageRectHeight) }
	};

	// Prepare rendering parameters of each view for swapchain texture arrays
	for (uint32_t i = 0; i < m_StereoViewCount; i++)
	{
		// Use the full size of the allocated swapchain image (could render smaller some frames to hit framerate)
		m_Swapchains[i].AcquireImage();
		m_ProjectionView[i] = { XR_TYPE_COMPOSITION_LAYER_PROJECTION_VIEW };
		m_ProjectionView[i].subImage.swapchain = m_Swapchains[i];
		m_ProjectionView[i].subImage.imageRect = image_rect;
	}

	// Begin the frame now
	XrFrameBeginInfo frame_begin_info{ XR_TYPE_FRAME_BEGIN_INFO };
	CHECK_XRCMD(xrBeginFrame(m_Handle, &frame_begin_info));

	// View state
	XrViewLocateInfo view_locate_info{ XR_TYPE_VIEW_LOCATE_INFO };
	view_locate_info.viewConfigurationType = m_PrimaryViewConfiguration;
	view_locate_info.displayTime = m_FrameState.predictedDisplayTime;
	view_locate_info.space = m_TrackingSpaces[m_TrackingOrigin];

	XrViewState view_state{ XR_TYPE_VIEW_STATE };
	uint32_t view_count;
	CHECK_XRCMD(xrLocateViews(m_Handle, &view_locate_info, &view_state, m_StereoViewCount, &view_count, m_Views));

	// Fill in the pose and FOV.
	for (uint32_t i = 0; i < m_StereoViewCount; i++)
	{
		m_ProjectionView[i].pose = m_Views[i].pose;
		m_ProjectionView[i].fov = m_Views[i].fov;
	}
}

void Session::EndFrame()
{
	// Release swap chain images
	for (uint32_t i = 0; i < m_StereoViewCount; i++)
	{
		m_Swapchains[i].ReleaseImage();
	}

	// The projection layer consists of projection layer views.
	XrCompositionLayerProjection layer{ XR_TYPE_COMPOSITION_LAYER_PROJECTION };
	layer.layerFlags = XR_COMPOSITION_LAYER_BLEND_TEXTURE_SOURCE_ALPHA_BIT;
	layer.views = m_ProjectionView;
	layer.viewCount = m_StereoViewCount;
	layer.space = m_TrackingSpaces[m_TrackingOrigin];

	// xrEndFrame can submit multiple layers. This sample submits one.
	std::vector<XrCompositionLayerBaseHeader*> layers;
	layers.push_back(reinterpret_cast<XrCompositionLayerBaseHeader*>(&layer));

	// Submit the composition layers for the predicted display time.
	XrFrameEndInfo frame_end_info{ XR_TYPE_FRAME_END_INFO };
	frame_end_info.displayTime = m_FrameState.predictedDisplayTime;
	frame_end_info.environmentBlendMode = m_EnvironmentBlendMode;
	frame_end_info.layers = layers.data();
	frame_end_info.layerCount = (uint32_t)layers.size();
	CHECK_XRCMD(xrEndFrame(m_Handle, &frame_end_info));
}

void Session::RecenterSpace()
{
	m_ViewRecenter = true;
	m_ViewRecenterTime = m_FrameState.predictedDisplayTime;
}

void Session::CopyImageToEye(size_t eye, ImageHandle handle)
{
	assert(eye < m_StereoViewCount);
	m_Swapchains[eye].CopyImage(handle);
}

XrPosef Session::GetHMDWorldPose(uint32_t index) const
{
	// Create rotation matrix
	const XrView& view = GetEyeView(index);

	// Create rotation matrix and apply it
	return view.pose;
	//return utils::PoseToCameraRelative(view.pose);
}

void Session::Initialize(XrInstance instance, XrSystemId systemId)
{
	PFN_xrGetD3D11GraphicsRequirementsKHR xrGetD3D11GraphicsRequirementsKHR = nullptr;
	CHECK_XRCMD(xrGetInstanceProcAddr(instance, "xrGetD3D11GraphicsRequirementsKHR",
									  reinterpret_cast<PFN_xrVoidFunction*>(&xrGetD3D11GraphicsRequirementsKHR)));

	XrGraphicsRequirementsD3D11KHR requirements{ XR_TYPE_GRAPHICS_REQUIREMENTS_D3D11_KHR };
	CHECK_XRCMD(xrGetD3D11GraphicsRequirementsKHR(instance, systemId, &requirements));

	// TODO: Should we actually do something with the graphics requirements?
	// Like, if you're running ffxiv_dx11.exe and you've gotten to this point that implies a few things

	XrGraphicsBindingD3D11KHR graphicsBinding{ XR_TYPE_GRAPHICS_BINDING_D3D11_KHR };
	graphicsBinding.device = game::graphics::kernel::Device::Get()->D3DDevice();

	XrSessionCreateInfo sessionCreateInfo{ XR_TYPE_SESSION_CREATE_INFO };
	sessionCreateInfo.next = reinterpret_cast<const XrBaseInStructure*>(&graphicsBinding);
	sessionCreateInfo.systemId = systemId;
	CHECK_XRCMD(xrCreateSession(instance, &sessionCreateInfo, &m_Handle));

	m_InputManager = std::make_unique<InputManager>(instance, *this);
}

void Session::SetBlendMode(XrInstance instance, XrSystemId systemId)
{
	// Query the list of supported environment blend modes for the current system.
	uint32_t count;
	CHECK_XRCMD(xrEnumerateEnvironmentBlendModes(instance, systemId, m_PrimaryViewConfiguration, 0, &count, nullptr));
	assert(count > 0);	// A system must support at least one environment blend mode.

	std::vector<XrEnvironmentBlendMode> blendModes(count);
	CHECK_XRCMD(xrEnumerateEnvironmentBlendModes(instance, systemId, m_PrimaryViewConfiguration, count, &count, blendModes.data()));

	// Pick the system's preferred one.
	m_EnvironmentBlendMode = blendModes[0];
}

void Session::SetDefaultViewConfiguration(XrInstance instance, XrSystemId systemId)
{
	assert(m_Handle != XR_NULL_HANDLE);

	XrSystemProperties properties{ XR_TYPE_SYSTEM_PROPERTIES };
	CHECK_XRCMD(xrGetSystemProperties(instance, systemId, &properties));

	uint32_t viewCount;
	CHECK_XRCMD(xrEnumerateViewConfigurationViews(instance, systemId, m_PrimaryViewConfiguration, 0, &viewCount, nullptr));
	assert(viewCount == m_StereoViewCount);

	std::vector<XrViewConfigurationView> configuration_views(viewCount, { XR_TYPE_VIEW_CONFIGURATION_VIEW });
	CHECK_XRCMD(xrEnumerateViewConfigurationViews(instance, systemId, m_PrimaryViewConfiguration, viewCount, &viewCount, configuration_views.data()));

	assert(configuration_views[0].recommendedImageRectWidth == configuration_views[1].recommendedImageRectWidth);
	assert(configuration_views[0].recommendedImageRectHeight == configuration_views[1].recommendedImageRectHeight);
	assert(configuration_views[0].recommendedSwapchainSampleCount == configuration_views[1].recommendedSwapchainSampleCount);

	m_ViewConfiguration = configuration_views[0];
	for (auto& view : m_Views)
		view.type = XR_TYPE_VIEW;

	Logf("Obtained view configuration: %u, %u", m_ViewConfiguration.recommendedImageRectWidth, m_ViewConfiguration.recommendedImageRectHeight);

	auto window = game::system::framework::Framework::Get()->Window();
	window->SetResizingEnabled(false);
	window->SetSize(m_ViewConfiguration.recommendedImageRectWidth, m_ViewConfiguration.recommendedImageRectHeight);
}

void Session::CreateSwapchains()	
{
	assert(m_Handle != XR_NULL_HANDLE);

	uint32_t width = m_ViewConfiguration.recommendedImageRectWidth;
	uint32_t height = m_ViewConfiguration.recommendedImageRectHeight;

	m_Swapchains.clear();
	m_Swapchains.reserve(m_StereoViewCount);

	for (uint32_t i = 0; i < m_StereoViewCount; ++i)
	{
		m_Swapchains.emplace_back(m_Handle, width, height, DXGI_FORMAT_R8G8B8A8_UNORM, XR_SWAPCHAIN_USAGE_SAMPLED_BIT | XR_SWAPCHAIN_USAGE_COLOR_ATTACHMENT_BIT);
	}
}

void Session::CreateSpaces()
{
	assert(m_Handle != XR_NULL_HANDLE);

	XrReferenceSpaceCreateInfo createInfo = { XR_TYPE_REFERENCE_SPACE_CREATE_INFO };
	createInfo.poseInReferenceSpace = { { 0.f, 0.f, 0.f, 1.f }, { 0.f, 0.f, 0.f } };
	createInfo.referenceSpaceType = XR_REFERENCE_SPACE_TYPE_VIEW;
	CHECK_XRCMD(xrCreateReferenceSpace(m_Handle, &createInfo, &m_ViewSpace));

	createInfo.referenceSpaceType = XR_REFERENCE_SPACE_TYPE_LOCAL;
	CHECK_XRCMD(xrCreateReferenceSpace(m_Handle, &createInfo, &m_OriginSpaces[xr::TrackingOrigin::EyeLevel]));
	CHECK_XRCMD(xrCreateReferenceSpace(m_Handle, &createInfo, &m_TrackingSpaces[xr::TrackingOrigin::EyeLevel]));

	createInfo.referenceSpaceType = XR_REFERENCE_SPACE_TYPE_STAGE;
	CHECK_XRCMD(xrCreateReferenceSpace(m_Handle, &createInfo, &m_OriginSpaces[xr::TrackingOrigin::FloorLevel]));
	CHECK_XRCMD(xrCreateReferenceSpace(m_Handle, &createInfo, &m_TrackingSpaces[xr::TrackingOrigin::FloorLevel]));
}

void Session::RecenterSpace(xr::TrackingOrigin origin)
{
	// Get the latest HMD pose
	XrPosef pose;
	if (!GetPose(pose, m_ViewSpace, &m_OriginSpaces[0], origin))
		return;

	// Construct the new origin pose
	XrPosef newOrigin = { { 0.f, 0.f, 0.f, 1.f }, { 0.f, 0.f, 0.f } };

	// Isolate yaw from pose since the floor is always gravity aligned
	XrQuaternionf& orientation = pose.orientation;
	orientation.x = 0.f;
	orientation.z = 0.f;
	float magnitude = sqrtf(orientation.y * orientation.y + orientation.w * orientation.w);
	orientation.y /= magnitude;
	orientation.w /= magnitude;

	// Create matrices
	XrVector3f scale{ 1.f, 1.f, 1.f };
	XrMatrix4x4f originMatrix;
	XrMatrix4x4f_CreateTranslationRotationScale(&originMatrix, &newOrigin.position, &newOrigin.orientation, &scale);
	XrMatrix4x4f offsetMatrix;
	XrMatrix4x4f_CreateTranslationRotationScale(&offsetMatrix, &pose.position, &pose.orientation, &scale);

	// Multiple and apply transformation to new origin
	XrMatrix4x4f poseMatrix;
	XrMatrix4x4f_Multiply(&poseMatrix, &originMatrix, &offsetMatrix);
	XrMatrix4x4f_GetTranslation(&newOrigin.position, &poseMatrix);
	XrMatrix4x4f_GetRotation(&newOrigin.orientation, &poseMatrix);

	// For floor level spaces we keep the height at the floor
	if (origin == xr::TrackingOrigin::FloorLevel)
		newOrigin.position.y = 0.0f;

	// Replace the tracking space with the newly calibrated one
	XrReferenceSpaceCreateInfo spaceInfo = { XR_TYPE_REFERENCE_SPACE_CREATE_INFO };
	spaceInfo.referenceSpaceType = static_cast<XrReferenceSpaceType>(XR_REFERENCE_SPACE_TYPE_LOCAL + origin);
	spaceInfo.poseInReferenceSpace = newOrigin;
	CHECK_XRCMD(xrDestroySpace(m_TrackingSpaces[origin]));
	CHECK_XRCMD(xrCreateReferenceSpace(m_Handle, &spaceInfo, &m_TrackingSpaces[origin]));

	// Done!
	m_ViewRecenter = false;
}
};	// namespace xr