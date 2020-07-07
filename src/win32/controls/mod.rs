mod button;
mod label;
mod progressbar;
mod slider;
mod util;
use winapi::shared::windef::*;

pub use button::Button;
pub use label::Label;
pub use progressbar::ProgressBar;
pub use slider::Slider;
pub use util::{ensure_control, send_message, set_window_pos};

pub trait Control {
    fn set_pos(&self, x: i32, y: i32, width: i32, height: i32);

    fn enable(&self);

    fn get_hwnd(&self) -> HWND;
}

trait WinControl {
    fn get_hwnd(&self) -> HWND;
}

// impl Control for &dyn WinControl {
//     fn set_pos(&self, x: i32, y: i32) {}

//     fn enable(&self) {}
// }
