use std::ptr::null_mut;
use winapi::shared::minwindef::*;
use winapi::shared::windef::*;

use super::super::{get_h_instance, win32_string, Win32Error, Win32Result};
use winapi::um::winuser;

/// Helper function to create a control, and raise an error otherwise.
pub fn ensure_control(
    dw_ex_style: DWORD,
    class_name: &str,
    title: &str,
    dw_style: DWORD,
    hwnd_parent: HWND,
) -> Win32Result<HWND> {
    let h_instance = get_h_instance();
    let class_name = win32_string(class_name);
    let title = win32_string(title);

    let width = 100;
    let height = 40;

    let hwnd = unsafe {
        winuser::CreateWindowExW(
            dw_ex_style,
            class_name.as_ptr(),
            title.as_ptr(),
            dw_style | winuser::WS_CHILD | winuser::WS_VISIBLE,
            winuser::CW_USEDEFAULT,
            winuser::CW_USEDEFAULT,
            width,
            height,
            hwnd_parent,
            null_mut(),
            h_instance,
            null_mut(),
        )
    };

    if hwnd.is_null() {
        return Err(Win32Error::from("No control created!"));
    }

    Ok(hwnd)
}

pub fn set_window_pos(hwnd: HWND, x: i32, y: i32, width: i32, height: i32) -> Win32Result {
    let res = unsafe { winuser::SetWindowPos(hwnd, std::ptr::null_mut(), x, y, width, height, 0) };

    if res == 0 {
        Err(Win32Error::from("SetWindowPos failed"))
    } else {
        Ok(())
    }
}

pub fn send_message(hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> Win32Result<LRESULT> {
    let res = unsafe { winuser::SendMessageW(hwnd, msg, wparam, lparam) };
    Ok(res)
}
