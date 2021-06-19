#include "xr/SessionManager.h"

#include "xr/XR.h"
#include "xr/XRUtils.h"
#include "xr/XRLinear.h"
#include "xr/Session.h"
#include "xr/InputManager.h"

#include "game/system/framework/Framework.h"

#include <string>
#include <vector>
#include <openxr/openxr.h>
#include <openxr/openxr_platform.h>
#include <cassert>

#include "Log.h"

namespace xr
{
SessionManager::SessionManager()
{
	// Back up the current window size.
	game::system::framework::Framework::Get()->Window()->GetSize(&m_OriginalWidth, &m_OriginalHeight);

	CreateInstance();
	InitializeSystem();
	InitializeSession();
}

SessionManager::~SessionManager()
{
	m_Session.reset();
	xrDestroyInstance(m_Instance);

	auto window = game::system::framework::Framework::Get()->Window();
	window->SetResizingEnabled(true);
	window->SetSize(m_OriginalWidth, m_OriginalHeight);
}

Session* SessionManager::GetSession() const
{
	// If our session exists but is not active, give downstream code a dead session so that they don't attempt to use it.
	if (m_Session && !m_Session->IsActive())
		return nullptr;
	return m_Session.get();
}

std::vector<const char*> SelectExtensions()
{
	uint32_t count;
	CHECK_XRCMD(xrEnumerateInstanceExtensionProperties(nullptr, 0, &count, nullptr));

	std::vector<XrExtensionProperties> properties(count, { XR_TYPE_EXTENSION_PROPERTIES });
	CHECK_XRCMD(xrEnumerateInstanceExtensionProperties(nullptr, count, &count, properties.data()));

	std::vector<const char*> extensions;

	// Add a specific extension to the list of extensions to be enabled, if it is supported.
	auto EnableExtensionIfSupported = [&](const char* name) {
		for (uint32_t i = 0; i < count; i++)
		{
			if (strcmp(properties[i].extensionName, name) == 0)
			{
				extensions.push_back(name);
				return true;
			}
		}

		return false;
	};

	bool success = true;
#ifdef XR_USE_GRAPHICS_API_D3D11
	success = EnableExtensionIfSupported(XR_KHR_D3D11_ENABLE_EXTENSION_NAME);
	assert(success);
#endif

	return extensions;
}

void SessionManager::CreateInstance()
{
	assert(m_Instance == XR_NULL_HANDLE);
	const std::vector<const char*> extensions = SelectExtensions();

	XrInstanceCreateInfo info{ XR_TYPE_INSTANCE_CREATE_INFO };
	info.enabledExtensionCount = static_cast<uint32_t>(extensions.size());
	info.enabledExtensionNames = extensions.data();
	info.applicationInfo = { "XIVR", 1, "", 1, XR_CURRENT_API_VERSION };

	CHECK_XRCMD(xrCreateInstance(&info, &m_Instance));
}

void SessionManager::InitializeSystem()
{
	assert(m_Instance != XR_NULL_HANDLE);
	assert(m_SystemId == XR_NULL_SYSTEM_ID);

	XrSystemGetInfo info{ XR_TYPE_SYSTEM_GET_INFO };
	info.formFactor = m_FormFactor;

	while (true)
	{
		XrResult result = xrGetSystem(m_Instance, &info, &m_SystemId);

		if (XR_SUCCEEDED(result))
		{
			Logf("Headset detected! System Id: %llu", m_SystemId);
			break;
		}
		else if (result == XR_ERROR_FORM_FACTOR_UNAVAILABLE)
		{
			Logf("No headset detected! Trying again in one second...\n");
			Sleep(1000);
		}
		else
		{
			CHECK_XRRESULT(result, "xrGetSystem");
		}
	}
}

void SessionManager::InitializeSession()
{
	assert(m_Instance != XR_NULL_HANDLE);
	assert(m_SystemId != XR_NULL_SYSTEM_ID);
	m_Session = std::make_unique<Session>(m_Instance, m_SystemId);
}

const XrEventDataBaseHeader* SessionManager::TryReadNextEvent()
{
	// It is sufficient to clear the just the XrEventDataBuffer header to XR_TYPE_EVENT_DATA_BUFFER
	XrEventDataBaseHeader* baseHeader = reinterpret_cast<XrEventDataBaseHeader*>(&m_EventDataBuffer);
	*baseHeader = { XR_TYPE_EVENT_DATA_BUFFER };
	const XrResult xr = xrPollEvent(m_Instance, &m_EventDataBuffer);
	if (xr == XR_SUCCESS)
	{
		if (baseHeader->type == XR_TYPE_EVENT_DATA_EVENTS_LOST)
		{
			const XrEventDataEventsLost* const eventsLost = reinterpret_cast<const XrEventDataEventsLost*>(baseHeader);
			Logf("%d events lost\n", eventsLost->lostEventCount);
		}

		return baseHeader;
	}
	if (xr == XR_EVENT_UNAVAILABLE)
	{
		return nullptr;
	}
	ExitWithError("xrPollEvent");
	return nullptr;
}

void SessionManager::PollEvents(bool& exit, bool& restart)
{
	exit = restart = false;

	// Process all pending messages.
	while (const XrEventDataBaseHeader* event = TryReadNextEvent())
	{
		switch (event->type)
		{
		case XR_TYPE_EVENT_DATA_INSTANCE_LOSS_PENDING: {
			const auto& instanceLossPending = *reinterpret_cast<const XrEventDataInstanceLossPending*>(event);
			Logf("XrEventDataInstanceLossPending by %lld\n", instanceLossPending.lossTime);
			exit = true;
			restart = true;
			return;
		}
		case XR_TYPE_EVENT_DATA_SESSION_STATE_CHANGED: {
			auto sessionStateChangedEvent = *reinterpret_cast<const XrEventDataSessionStateChanged*>(event);
			HandleSessionStateChangedEvent(sessionStateChangedEvent, exit, restart);
			break;
		}
		case XR_TYPE_EVENT_DATA_INTERACTION_PROFILE_CHANGED:
			Logf("Interaction profile changed\n");
			if (auto inputManager = GetInputManager())
			{
				for (auto const& p1 : inputManager->GetActionSets())
				{
					Logf("%s\n", p1.second.GetName().c_str());
					for (auto const& p2 : p1.second.GetActions())
						p2.second.LogBinding(*GetSession());
				}
			}
			break;
		case XR_TYPE_EVENT_DATA_REFERENCE_SPACE_CHANGE_PENDING:
			auto spaceChangeEvent = *reinterpret_cast<const XrEventDataReferenceSpaceChangePending*>(event);
			HandleReferenceSpaceChangeEvent(spaceChangeEvent);
			break;
		default: {
			Logf("Ignoring event type %d\n", event->type);
			break;
		}
		}
	}
}

void SessionManager::HandleSessionStateChangedEvent(const XrEventDataSessionStateChanged& event, bool& exit, bool& restart)
{
	assert(m_Session != nullptr);

	const XrSessionState oldState = m_SessionState;
	m_SessionState = event.state;

	Logf("XrEventDataSessionStateChanged: state %s->%s session=%p time=%lld\n", to_string(oldState),
		 to_string(m_SessionState), event.session, event.time);

	if ((event.session != XR_NULL_HANDLE) && (event.session != *m_Session))
	{
		Logf("XrEventDataSessionStateChanged for unknown session\n");
		return;
	}

	switch (m_SessionState)
	{
	case XR_SESSION_STATE_READY: {
		m_Session->Begin();
		break;
	}
	case XR_SESSION_STATE_STOPPING: {
		m_Session->End();
		break;
	}
	case XR_SESSION_STATE_EXITING: {
		exit = true;
		restart = false;
		break;
	}
	case XR_SESSION_STATE_LOSS_PENDING: {
		exit = true;
		restart = true;
		break;
	}
	default:
		break;
	}
}

void SessionManager::HandleReferenceSpaceChangeEvent(const XrEventDataReferenceSpaceChangePending& event)
{
	assert(m_Session != nullptr);

	if ((event.session != XR_NULL_HANDLE) && (event.session != *m_Session))
	{
		Logf("XrEventDataReferenceSpaceChangePending for unknown session\n");
		return;
	}

	m_Session->RecenterSpace();
}
};	// namespace xr
