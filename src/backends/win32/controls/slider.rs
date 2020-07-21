use winapi::shared::windef::*;

use super::super::{get_util_window, Win32Result};
use super::{ensure_control, WinControl};
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

impl WinControl for Slider {
    fn get_hwnd(&self) -> HWND {
        self.hwnd
    }
}
