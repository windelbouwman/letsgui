use super::controls::Control;
use std::ptr::null_mut;
use std::sync::{Arc, Mutex};
use winapi::shared::minwindef::*;
use winapi::shared::windef::*;
use winapi::um::winuser;

use super::{get_h_instance, win32_string, Win32Error, Win32Result};

const CLASS_NAME: &str = "pietjepuk";

/// Register windows class for top level window.
pub fn init_window(h_instance: HMODULE) -> Win32Result {
    println!("Initializing window");

    let class_name = win32_string(CLASS_NAME);
    unsafe {
        let wc = winuser::WNDCLASSEXW {
            cbSize: std::mem::size_of::<winuser::WNDCLASSEXW>() as u32,
            style: winuser::CS_OWNDC | winuser::CS_HREDRAW | winuser::CS_VREDRAW,
            lpfnWndProc: Some(window_proc),
            cbClsExtra: 0,
            cbWndExtra: std::mem::size_of::<Box<Window>>() as i32,
            hInstance: h_instance,
            hIcon: winuser::LoadIconW(null_mut(), winuser::IDI_APPLICATION),
            hCursor: winuser::LoadCursorW(null_mut(), winuser::IDC_ARROW),
            hbrBackground: winuser::COLOR_WINDOWFRAME as HBRUSH,
            lpszMenuName: null_mut(),
            lpszClassName: class_name.as_ptr(),
            hIconSm: winuser::LoadIconW(null_mut(), winuser::IDI_APPLICATION),
        };

        if winuser::RegisterClassExW(&wc) == 0 {
            return Err(Win32Error::from("Window registration failed"));
        }
    }

    Ok(())
}

pub struct Window {
    title: String,

    hwnd: Mutex<Option<HWND>>,

    children: Mutex<Vec<Box<dyn Control>>>,
}

impl Window {
    pub fn new(title: &str) -> Win32Result<Arc<Self>> {
        let h_instance = get_h_instance();
        let class_name = win32_string(CLASS_NAME);

        let window = Arc::new(Window {
            title: title.to_owned(),
            hwnd: Default::default(),
            children: Default::default(),
        });
        let title = win32_string(title);

        let lp_param = Arc::into_raw(window.clone());
        let lp_param = lp_param as *mut winapi::ctypes::c_void;
        println!("W00T: {:?}", lp_param);
        let hwnd = unsafe {
            winuser::CreateWindowExW(
                winuser::WS_EX_CLIENTEDGE,
                class_name.as_ptr(),
                title.as_ptr(),
                winuser::WS_OVERLAPPEDWINDOW | winuser::WS_VISIBLE,
                winuser::CW_USEDEFAULT,
                winuser::CW_USEDEFAULT,
                winuser::CW_USEDEFAULT,
                winuser::CW_USEDEFAULT, // nHeight
                null_mut(),             // hWndParent
                null_mut(),             // hMenu
                h_instance,
                lp_param, // lpParam
            )
        };

        if hwnd.is_null() {
            return Err(Win32Error::from("No window created!"));
        }

        window.set_hwnd(hwnd);

        Ok(window)
    }

    pub fn add_control(&self, control: Box<dyn Control>) {
        // control. self.hwnd()
        unsafe {
            winuser::SetParent(control.get_hwnd(), self.hwnd());
        }
        self.children.lock().unwrap().push(control);
    }

    fn set_hwnd(&self, hwnd: HWND) {
        self.hwnd.lock().unwrap().replace(hwnd);
    }
    pub fn hwnd(&self) -> HWND {
        self.hwnd.lock().unwrap().unwrap()
    }

    fn do_layout(&self, size: Size) {
        let children = &*self.children.lock().unwrap();
        if !children.is_empty() {
            let padding = 5;
            let dy = size.height / children.len() as i32;
            let height = (dy - (padding * 2)).max(0);
            let width = (size.width - padding * 2).max(0);
            let mut y = 0;
            // layout child elements.
            for child in children {
                // println!("REsize child!!!!!!!!!");
                child.set_pos(padding, y + padding, width, height);
                y += dy;
            }
        }
    }
}

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    // println!("MSG: {}", msg);
    match msg {
        winuser::WM_CREATE => {
            // println!("CREATORZZ");
            let cs: *mut winuser::CREATESTRUCTW = lparam as *mut winuser::CREATESTRUCTW;
            // let window =
            let p = (*cs).lpCreateParams as winapi::shared::basetsd::LONG_PTR;
            winuser::SetWindowLongPtrW(hwnd, winuser::GWLP_USERDATA, p);
            // println!("CREATORZ: {:?}", p);
        }
        winuser::WM_CLOSE => {
            winuser::DestroyWindow(hwnd);
            return 0;
        }
        winuser::WM_DESTROY => {
            winuser::PostQuitMessage(0);
            return 0;
        }
        winuser::WM_SIZE => {
            // println!("SIZEEEE!");
            let p = winuser::GetWindowLongPtrW(hwnd, winuser::GWLP_USERDATA);
            // println!("POINT0R: {:?}", p);
            let w = p as *const Window;
            // *w.children
            let r = get_window_size(hwnd).unwrap();
            let width = r.right - r.left;
            let height = r.bottom - r.top;
            let size = Size { width, height };
            // println!("size={:?}", size);
            w.as_ref().unwrap().do_layout(size);
        }
        winuser::WM_WINDOWPOSCHANGED => {
            // println!("Window pos changed!");
        }
        _ => {}
    }
    winuser::DefWindowProcW(hwnd, msg, wparam, lparam)
}

#[derive(Debug)]
struct Size {
    width: i32,
    height: i32,
}

fn get_window_size(hwnd: HWND) -> Result<RECT, Win32Error> {
    unsafe {
        let mut rect: RECT = std::mem::uninitialized();
        if winuser::GetWindowRect(hwnd, &mut rect) == 0 {
            Err(Win32Error::from("Error in GetClientRect"))
        } else {
            Ok(rect)
            // Size {
            //     width: rect.width,
            //     height: rect.height,
            // })
        }
    }
}
