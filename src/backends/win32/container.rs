//! A layouting container for win32 controls.

use std::ptr::null_mut;
use winapi::shared::minwindef::*;
use winapi::shared::windef::*;
use winapi::um::winuser;

use super::controls::ensure_control;
use super::{win32_string, Win32Error, Win32Result};

const CLASS_NAME: &str = "letsgui_container";

/// Register container window class.
pub fn init_container(h_instance: HMODULE) -> Win32Result {
    info!("Initializing container");
    let class_name = win32_string(CLASS_NAME);

    unsafe {
        let wc = winuser::WNDCLASSW {
            style: 0,
            lpfnWndProc: Some(window_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: h_instance,
            hIcon: winuser::LoadIconW(null_mut(), winuser::IDI_APPLICATION),
            hCursor: winuser::LoadCursorW(null_mut(), winuser::IDC_ARROW),
            hbrBackground: winuser::COLOR_WINDOWFRAME as HBRUSH,
            lpszMenuName: null_mut(),
            lpszClassName: class_name.as_ptr(),
        };

        if winuser::RegisterClassW(&wc) == 0 {
            return Err(Win32Error::from("Window registration failed"));
        }
    }

    Ok(())
}

/// A vbox container for layout of child nodes.
pub struct Container {
    hwnd: HWND,
}

impl Container {
    pub fn new(parent: HWND) -> Win32Result<Self> {
        let hwnd = ensure_control(winuser::WS_EX_CONTROLPARENT, CLASS_NAME, "", 0, parent)?;

        let container = Container { hwnd };

        Ok(container)
    }
}

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        winuser::WM_CREATE => {
            info!("Container created!");
        }
        winuser::WM_WINDOWPOSCHANGED => {
            trace!("Container pos changed!");
        }
        _ => {}
    }
    winuser::DefWindowProcW(hwnd, msg, wparam, lparam)
}
