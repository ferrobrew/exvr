use macros::game_class;

use bindings::Windows::Win32::Foundation::{HWND, LPARAM, RECT, WPARAM};
use bindings::Windows::Win32::UI::WindowsAndMessaging::*;

game_class!(Window, {
    fields: {
        [0x18] handle: HWND
    }
});

impl Window {
    pub fn get_size(&self) -> (u32, u32) {
        let mut r: RECT = unsafe { std::mem::zeroed() };
        unsafe { GetClientRect(self.handle(), &mut r) };
        (r.right as u32, r.bottom as u32)
    }

    pub fn set_size(&mut self, size: (u32, u32)) {
        let mut r = RECT {
            left: 0,
            top: 0,
            right: size.0 as i32,
            bottom: size.1 as i32,
        };
        unsafe {
            AdjustWindowRect(
                &mut r,
                WINDOW_STYLE(GetWindowLongA(self.handle(), GWL_STYLE) as u32),
                false,
            )
        };
        r.bottom -= r.top;
        r.right -= r.left;
        unsafe {
            SetWindowPos(
                self.handle(),
                HWND::NULL,
                0,
                0,
                r.right,
                r.bottom,
                SWP_NOACTIVATE | SWP_NOMOVE | SWP_NOZORDER | SWP_FRAMECHANGED,
            );
            SendMessageA(self.handle(), WM_EXITSIZEMOVE, WPARAM(0), LPARAM(0));
        }
    }

    pub fn set_resizing_enabled(&mut self, resizable: bool) {
        let mask = WS_MINIMIZEBOX | WS_MAXIMIZEBOX | WS_SIZEBOX;
        let current_style = unsafe { GetWindowLongA(self.handle(), GWL_STYLE) } as u32;
        let new_style = if resizable {
            current_style | mask.0
        } else {
            current_style & !mask.0
        };
        unsafe {
            SetWindowLongA(self.handle(), GWL_STYLE, new_style as i32);
        }
    }
}
