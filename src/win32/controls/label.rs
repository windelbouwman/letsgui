use winapi::shared::windef::*;

use super::super::{get_util_window, Win32Result};
use super::util::set_window_pos;
use super::{ensure_control, Control, WinControl};
use winapi::um::winuser;

pub struct Label {
    hwnd: HWND,
}

impl Label {
    pub fn new(caption: &str) -> Win32Result<Self> {
        let hwnd_parent: HWND = get_util_window();

        let hwnd = ensure_control(
            0,
            "static",
            caption,
            winuser::SS_LEFTNOWORDWRAP | winuser::SS_NOPREFIX,
            hwnd_parent,
        )?;

        Ok(Label { hwnd })
    }
}

impl Control for Label {
    fn set_pos(&self, x: i32, y: i32, width: i32, height: i32) {
        set_window_pos(self.hwnd, x, y, width, height).unwrap();
    }

    fn enable(&self) {}
    fn get_hwnd(&self) -> HWND {
        self.hwnd
    }
}

impl WinControl for Label {
    fn get_hwnd(&self) -> HWND {
        self.hwnd
    }
}
