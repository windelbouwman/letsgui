use winapi::shared::windef::*;

use super::super::{get_util_window, Win32Result};
use super::{ensure_control, send_message, WinControl};
use winapi::um::commctrl;
use winapi::um::winuser;

pub struct ProgressBar {
    hwnd: HWND,
}

impl ProgressBar {
    pub fn new() -> Win32Result<Self> {
        let hwnd_parent: HWND = get_util_window();

        let hwnd = ensure_control(
            0,
            "msctls_progress32",
            "",
            commctrl::PBS_SMOOTH,
            hwnd_parent,
        )?;

        Ok(ProgressBar { hwnd })
    }

    pub fn set_value(&self, value: usize) -> Win32Result {
        let _res = send_message(self.hwnd, commctrl::PBM_SETPOS, value, 0)?;
        Ok(())
    }
}

impl WinControl for ProgressBar {
    fn get_hwnd(&self) -> HWND {
        self.hwnd
    }
}
