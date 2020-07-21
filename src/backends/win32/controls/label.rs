use winapi::shared::windef::*;

use super::super::{get_util_window, Win32Result};
use super::{ensure_control, WinControl};
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

impl WinControl for Label {
    fn get_hwnd(&self) -> HWND {
        self.hwnd
    }
}
