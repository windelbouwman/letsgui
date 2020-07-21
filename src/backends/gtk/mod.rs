//! GTK backend

// TODO: implement all

mod application;
mod controls;
mod window;

use controls::GtkControl;
pub use controls::MyError;
pub use controls::{Button, Entry, Label, ProgressBar, Slider};

pub use window::Window;

pub use application::{letsgui, Application};
