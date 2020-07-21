mod button;
mod entry;
mod label;
mod progressbar;
mod slider;

pub use button::Button;
pub use entry::Entry;
pub use label::Label;
pub use progressbar::ProgressBar;
pub use slider::Slider;

pub type MyError = String;

pub trait GtkControl {
    fn get_widget(&self) -> gtk::Widget;
}
