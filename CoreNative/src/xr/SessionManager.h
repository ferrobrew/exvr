#pragma once

#include <memory>
#include <openxr/openxr.h>

#include <wtypes.h>

namespace xr {
class SessionManager {
public:
    SessionManager();
    ~SessionManager();

    class Session* GetSession() const;
    void PollEvents(bool& exit, bool& restart);

private:
    void CreateInstance();
    void InitializeSystem();
    void InitializeSession();

    const XrEventDataBaseHeader* TryReadNextEvent();
    void HandleSessionStateChangedEvent(const XrEventDataSessionStateChanged& event, bool& exit, bool& restart);
    void HandleReferenceSpaceChangeEvent(const XrEventDataReferenceSpaceChangePending& event);

    constexpr static XrFormFactor m_FormFactor{XR_FORM_FACTOR_HEAD_MOUNTED_DISPLAY};

    XrInstance m_Instance{XR_NULL_HANDLE};
    XrSystemId m_SystemId{XR_NULL_SYSTEM_ID};
    std::unique_ptr<class Session> m_Session;

    XrEventDataBuffer m_EventDataBuffer;
    XrSessionState m_SessionState{XR_SESSION_STATE_UNKNOWN};

    uint32_t m_OriginalWidth;
	uint32_t m_OriginalHeight;
};
};
