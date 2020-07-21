use super::{GtkControl, MyError};
use gtk::prelude::*;

pub struct Slider {
    inner: gtk::Label,
}

impl Slider {
    pub fn new() -> Result<Self, MyError> {
        // gtk::Slider::new();
        let inner = gtk::Label::new(Some("TODO: slider"));
        Ok(Slider { inner })
    }
}

impl GtkControl for Slider {
    fn get_widget(&self) -> gtk::Widget {
        self.inner.clone().upcast::<gtk::Widget>()
    }
}
