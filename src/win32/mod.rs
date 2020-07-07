//! Windows Forms support

use std::sync::Arc;
use winapi::um::winuser;

mod container;
mod controls;
mod error;
mod util;
mod util_window;
mod window;

use container::{init_container, Container};
use controls::Control;
pub use controls::{Button, Label, ProgressBar, Slider};
pub use error::{Win32Error, Win32Result};
use util_window::{get_util_window, init_util_window};
use window::init_window;
pub use window::Window;

use util::{get_h_instance, win32_string};

pub fn initialize() -> Win32Result {
    let h_instance = get_h_instance();
    println!("Initializing...");

    // init components (register window classes)
    init_container(h_instance)?;
    init_window(h_instance)?;
    init_util_window(h_instance)?;

    Ok(())
}

pub fn run_event_loop(window: Arc<Window>) {
    unsafe {
        let mut msg: winuser::MSG = std::mem::uninitialized();
        loop {
            // let msg =
            if winuser::GetMessageW(&mut msg, window.hwnd(), 0, 0) > 0 {
                // println!("MSG: {:?}", msg.message);
                winuser::TranslateMessage(&msg);
                winuser::DispatchMessageW(&msg);
            } else {
                break;
            }
        }
    }
}
