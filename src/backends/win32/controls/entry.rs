use winapi::shared::windef::*;

use super::super::{get_util_window, Win32Result};
use super::{ensure_control, WinControl};
use winapi::um::winuser;

pub struct Entry {
    hwnd: HWND,
}

impl Entry {
    pub fn new() -> Win32Result<Self> {
        let hwnd_parent: HWND = get_util_window();

        let hwnd = ensure_control(
            0,
            "edit",
            "",
            winuser::ES_AUTOHSCROLL
                | winuser::ES_LEFT
                | winuser::ES_NOHIDESEL
                | winuser::WS_TABSTOP,
            hwnd_parent,
        )?;

        Ok(Entry { hwnd })
    }
}

impl WinControl for Entry {
    fn get_hwnd(&self) -> HWND {
        self.hwnd
    }
}
