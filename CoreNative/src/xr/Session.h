#pragma once

#include <memory>
#include <openxr/openxr.h>

#include "xr/Swapchain.h"

namespace xr {
enum TrackingOrigin { EyeLevel, FloorLevel, Count };

class Session {
public:
    operator XrSession() const { return m_Handle; }
    bool IsActive() const { return m_Active; }

    Session(XrInstance system, XrSystemId systemId);
    ~Session();

    const bool HasViewConfigurationChanged() const;
    const XrViewConfigurationView& GetViewConfiguration() const;
    const XrView& GetEyeView(uint32_t eye) const;
    const uint32_t GetEyeWidth() const;
    const uint32_t GetEyeHeight() const;
    const XrFovf GetEyeFOV(size_t index) const;
    class InputManager* GetInputManager() const;

    bool GetPose(XrPosef& pose, XrSpace space) const;
    bool GetPose(XrPosef& pose, XrSpace space, const XrSpace* spaces, TrackingOrigin origin) const;
    XrPosef GetHMDWorldPose(uint32_t index) const;

    void Begin();
    void End();

    // takes place before any logic or rendering
    void BeginTick();
    void BeginFrame();
    void EndFrame();
    void RecenterSpace();
    void CopyImageToEye(size_t eye, ImageHandle handle);

private:
    void Initialize(XrInstance instance, XrSystemId systemId);
    void SetBlendMode(XrInstance instance, XrSystemId systemId);
    void SetDefaultViewConfiguration(XrInstance instance, XrSystemId systemId);
    void CreateSwapchains();
    void CreateSpaces();
    void RecenterSpace(TrackingOrigin origin);

    constexpr static XrViewConfigurationType m_PrimaryViewConfiguration{XR_VIEW_CONFIGURATION_TYPE_PRIMARY_STEREO};
    constexpr static uint32_t m_StereoViewCount = 2;

    bool m_Active = false;

    XrSession m_Handle{XR_NULL_HANDLE};
    bool m_ViewConfigurationChanged{true};
    XrViewConfigurationView m_ViewConfiguration{XR_TYPE_VIEW_CONFIGURATION_VIEW};
    XrEnvironmentBlendMode m_EnvironmentBlendMode{XR_ENVIRONMENT_BLEND_MODE_OPAQUE};
    XrFrameState m_FrameState{XR_TYPE_FRAME_STATE};
    std::vector<Swapchain> m_Swapchains;
    XrView m_Views[m_StereoViewCount];
    XrCompositionLayerProjectionView m_ProjectionView[m_StereoViewCount];

    bool m_ViewRecenter{true};
    XrTime m_ViewRecenterTime{0};
    XrSpace m_ViewSpace{XR_NULL_HANDLE};
    XrSpace m_OriginSpaces[TrackingOrigin::Count]{XR_NULL_HANDLE, XR_NULL_HANDLE};
    XrSpace m_TrackingSpaces[TrackingOrigin::Count]{XR_NULL_HANDLE, XR_NULL_HANDLE};
    xr::TrackingOrigin m_TrackingOrigin{xr::TrackingOrigin::EyeLevel};
    std::unique_ptr<class InputManager> m_InputManager;
};
};
