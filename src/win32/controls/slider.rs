use winapi::shared::windef::*;

use super::super::{get_util_window, Win32Result};
use super::{ensure_control, set_window_pos, Control};
use winapi::um::commctrl;
use winapi::um::winuser;

pub struct Slider {
    hwnd: HWND,
}

impl Slider {
    pub fn new() -> Win32Result<Self> {
        let hwnd_parent: HWND = get_util_window();

        let hwnd = ensure_control(
            0,
            "msctls_trackbar32",
            "",
            commctrl::TBS_HORZ
                | commctrl::TBS_TOOLTIPS
                | commctrl::TBS_TRANSPARENTBKGND
                | winuser::WS_TABSTOP,
            hwnd_parent,
        )?;

        Ok(Slider { hwnd })
    }
}

impl Control for Slider {
    fn set_pos(&self, x: i32, y: i32, width: i32, height: i32) {
        set_window_pos(self.hwnd, x, y, width, height).unwrap();
    }

    fn enable(&self) {}

    fn get_hwnd(&self) -> HWND {
        self.hwnd
    }
}
