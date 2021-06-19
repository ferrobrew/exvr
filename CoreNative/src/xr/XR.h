#pragma once

#include <memory>
#include <functional>

namespace xr
{
class Session* GetSession();
class InputManager* GetInputManager();
class SessionManager* GetSessionManager();

bool IsInitialized();
void Initialize(bool enable);
void PollEvents(bool& exit, bool& restart);
void Shutdown();

void PreTick();
void Render(const std::function<void()>& fn);
void RenderEyes();
void PostTick();

float GetHFOVApproximation();
float GetScreenWidth();
float GetScreenHeight();
};	// namespace xr
