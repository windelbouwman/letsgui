//! Util window.
//!
//! Parent window, if no parent provided.

use winapi::shared::minwindef::*;
use winapi::shared::windef::*;
use winapi::um::winuser;

use std::cell::RefCell;
use std::ptr::null_mut;

use super::{win32_string, Win32Error, Win32Result};

const CLASS_NAME: &str = "util_windowClass";

thread_local!(static STORED_HWND: RefCell<Option<HWND>> = Default::default());

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    // return 0;
    winuser::DefWindowProcW(hwnd, msg, wparam, lparam)
}

pub fn init_util_window(h_instance: HMODULE) -> Win32Result {
    println!("Initializing util window");

    let class_name = win32_string(CLASS_NAME);

    unsafe {
        let wc = winuser::WNDCLASSW {
            style: 0,
            lpfnWndProc: Some(window_proc), // TODO: might be nice to add? ==> CRUCIAL, otherwise crash..
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: h_instance,
            hIcon: winuser::LoadIconW(null_mut(), winuser::IDI_APPLICATION),
            hCursor: winuser::LoadCursorW(null_mut(), winuser::IDC_ARROW),
            hbrBackground: (winuser::COLOR_BTNFACE + 1) as HBRUSH,
            lpszMenuName: null_mut(),
            lpszClassName: class_name.as_ptr(),
        };

        if winuser::RegisterClassW(&wc) == 0 {
            return Err(Win32Error::from("RegisterClassW failed horribly"));
        }
    }

    let title = win32_string("utility window title");

    println!("Creating util window");

    let hwnd = unsafe {
        winuser::CreateWindowExW(
            0,
            class_name.as_ptr(),
            title.as_ptr(),
            winuser::WS_OVERLAPPEDWINDOW,
            0,
            0,
            100,
            100,        // nHeight
            null_mut(), // hWndParent
            null_mut(), // hMenu
            h_instance,
            null_mut(), // lpParam
        )
    };

    // println!("Res = {:?}", hwnd);

    if hwnd.is_null() {
        // winuser::GetLastError();
        let err_code = unsafe { winapi::um::errhandlingapi::GetLastError() };
        println!("Last error: {}", err_code);
        // println!("OS error: {:?}", std::io::Error::last_os_error());
        return Err(Win32Error::from("No util window created!"));
    }

    STORED_HWND.with(|h| {
        h.replace(Some(hwnd));
    });

    println!("Util window creat0rred");

    Ok(())
}

pub fn get_util_window() -> HWND {
    STORED_HWND.with(|h| h.borrow().unwrap())
}

// pub fn get_default_parent_hwnd() -> HWND {
//     STORED_HWND.borrow().unwrap()
// }
