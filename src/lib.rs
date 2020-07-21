//! letsgui

#[macro_use]
extern crate log;

pub mod backends;

#[cfg(feature = "win32-backend")]
pub use backends::win32::letsgui;

#[cfg(feature = "gtk-backend")]
pub use backends::gtk::letsgui;

#[cfg(feature = "win32-backend")]
pub use backends::win32::{Button, Entry, Label, ProgressBar, Slider, Win32Result, Window};

#[cfg(feature = "gtk-backend")]
pub use backends::gtk::{Button, Entry, Label, ProgressBar, Slider, Window};

#[cfg(feature = "gtk-backend")]
pub use backends::gtk::MyError;

#[cfg(feature = "gtk-backend")]
pub type Win32Result = Result<(), MyError>;
