use winapi::shared::windef::*;

use super::super::{get_util_window, Win32Result};
use super::{ensure_control, set_window_pos, Control, WinControl};
use winapi::um::winuser;

pub struct Button {
    hwnd: HWND,
    clicked_handler: Option<&'static dyn Fn(Self)>,
}

impl Button {
    pub fn new(caption: &str) -> Win32Result<Self> {
        let hwnd_parent: HWND = get_util_window();
        let hwnd = ensure_control(
            0,
            "button",
            caption,
            winuser::BS_PUSHBUTTON | winuser::WS_TABSTOP,
            hwnd_parent,
        )?;

        Ok(Button {
            hwnd,
            clicked_handler: None,
        })
    }
}

impl Button {
    pub fn on_clicked<F>(&self, f: &'static dyn Fn(&Self))
    where
        F: Fn(&Self),
    {
        // self.clicked_handler = Some(f);
    }
}

impl WinControl for Button {
    fn get_hwnd(&self) -> HWND {
        self.hwnd
    }
}

impl Control for Button {
    fn set_pos(&self, x: i32, y: i32, width: i32, height: i32) {
        set_window_pos(self.hwnd, x, y, width, height).unwrap();
    }

    fn enable(&self) {}
    fn get_hwnd(&self) -> HWND {
        self.hwnd
    }
}
