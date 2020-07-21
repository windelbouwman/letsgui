use std::sync::Arc;
use winapi::um::winuser;

use super::get_h_instance;
use super::init_container;
use super::init_util_window;
use super::init_window;
use super::{Win32Result, Window};

pub struct Application {}

impl Application {
    pub fn new() -> Win32Result<Self> {
        initialize()?;
        Ok(Application {})
    }

    pub fn run(&self, window: Arc<Window>) {
        run_event_loop(window);
    }
}

fn initialize() -> Win32Result {
    let h_instance = get_h_instance();
    info!("Initializing...");

    // init components (register window classes)
    init_container(h_instance)?;
    init_window(h_instance)?;
    init_util_window(h_instance)?;

    Ok(())
}

fn run_event_loop(window: Arc<Window>) {
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

pub fn letsgui<F>(init_func: F) -> Win32Result
where
    F: Fn() -> Win32Result<Arc<Window>> + 'static,
{
    initialize()?;
    let window = init_func()?;
    run_event_loop(window);
    Ok(())
}
