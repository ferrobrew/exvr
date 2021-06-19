#pragma once

#include "game/Util.h"
#include "game/Addresses.h"

#include <wtypes.h>

namespace game::system::framework
{
class Window
{
public:
	OFFSET_PROPERTY(HWND, Handle, 0x18);

	void GetSize(uint32_t* width, uint32_t* height)
	{
		RECT r;
		GetClientRect(Handle(), &r);
		*width = r.right;
		*height = r.bottom;
	}

	void SetSize(uint32_t width, uint32_t height)
	{
		RECT r = { 0, 0, LONG(width), LONG(height) };
		AdjustWindowRect(&r, GetWindowLong(Handle(), GWL_STYLE), FALSE);
		// AdjustWindowRect gives these negative values, so we move them to the size
		r.bottom -= r.top;
		r.right -= r.left;
		SetWindowPos(Handle(), 0, 0, 0, r.right, r.bottom, SWP_NOACTIVATE | SWP_NOMOVE | SWP_NOZORDER | SWP_NOZORDER | SWP_FRAMECHANGED);
		SendMessage(Handle(), WM_EXITSIZEMOVE, 0, 0);
	}

	void SetResizingEnabled(bool resizable)
	{
		uint32_t mask = WS_MINIMIZEBOX | WS_MAXIMIZEBOX | WS_SIZEBOX;
		auto currentStyle = GetWindowLong(Handle(), GWL_STYLE);
		SetWindowLong(Handle(), GWL_STYLE, resizable ? (currentStyle | mask) : (currentStyle & ~mask));
	}
};

class Framework
{
public:
	static Framework* Get()
	{
		return game::DerefBaseRelativePtr<Framework*>(game::offsets::globals::Framework);
	}

	OFFSET_PROPERTY(framework::Window*, Window, 0x7A8);
};
}  // namespace game::system::framework