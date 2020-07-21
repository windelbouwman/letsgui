//! Windows Forms support

mod application;
mod container;
mod controls;
mod error;
mod util;
mod util_window;
mod window;

pub use application::{letsgui, Application};
use container::{init_container, Container};
use controls::Control;
pub use controls::{Button, Entry, Label, ProgressBar, Slider};
pub use error::{Win32Error, Win32Result};
use util_window::{get_util_window, init_util_window};
use window::init_window;
pub use window::Window;

use util::{get_h_instance, win32_string};
