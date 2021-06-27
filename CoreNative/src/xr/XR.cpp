#include "xr/XR.h"

#include "xr/InputManager.h"
#include "xr/Session.h"
#include "xr/SessionManager.h"
#include "xr/StereoRender.h"
#include "xr/XRLinear.h"

#include "game/graphics/kernel/Device.h"
#include "game/graphics/kernel/SwapChain.h"

namespace xr
{
static StereoRender stereoRender;
static std::unique_ptr<SessionManager> sessionManager;

Session* GetSession()
{
	if (!sessionManager)
	{
		return nullptr;
	}

	return sessionManager->GetSession();
}

InputManager* GetInputManager()
{
	auto session = GetSession();

	if (!session)
	{
		return nullptr;
	}

	return session->GetInputManager();
}

SessionManager* GetSessionManager()
{
	return sessionManager.get();
}

bool IsInitialized()
{
	return sessionManager != nullptr;
};

void Initialize(bool enable)
{
	stereoRender.Init();

	if (enable)
		sessionManager = std::make_unique<SessionManager>();
};

void PollEvents(bool& exit, bool& restart)
{
#ifdef OPTICK
	OPTICK_EVENT();
#endif
	if (sessionManager)
	{
		sessionManager->PollEvents(exit, restart);
	}
}

void PreTick()
{
#ifdef OPTICK
	OPTICK_EVENT();
#endif
	if (auto session = GetSession())
	{
		session->BeginTick();

		if (session->HasViewConfigurationChanged())
		{
			stereoRender.Destroy();
			stereoRender.Init();
		}

		session->BeginFrame();
	}
};

void PostTick()
{
#ifdef OPTICK
	OPTICK_EVENT();
#endif
	if (auto session = GetSession())
	{
		session->EndFrame();
	}
};

void Shutdown()
{
	sessionManager.reset();
}

float GetHFOVApproximation()
{
	if (auto session = xr::GetSession())
	{
		auto fovLeft = session->GetEyeFOV(0);
		auto fovRight = session->GetEyeFOV(1);

		// Left measurements are negative, so right - left == abs(left) + abs(right)
		return std::min((fovRight.angleRight - fovLeft.angleLeft) * (MATH_PI / 180.0f), 180.0f);
	}

	return 90.0f;
}

float GetScreenWidth()
{
	if (auto session = xr::GetSession())
	{
		return float(session->GetEyeWidth());
	}

	return float(game::graphics::kernel::Device::Get()->SwapChain()->Width());
}

float GetScreenHeight()
{
	if (auto session = xr::GetSession())
	{
		return float(session->GetEyeHeight());
	}

	return float(game::graphics::kernel::Device::Get()->SwapChain()->Height());
}
};	// namespace xr