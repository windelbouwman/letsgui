use gtk::prelude::*;

use super::{GtkControl, MyError};

pub struct Button {
    inner: gtk::Button,
}

impl Button {
    pub fn new(label: &str) -> Result<Self, MyError> {
        let inner = gtk::Button::new();
        inner.set_label(label);
        Ok(Button { inner })
    }
}

impl GtkControl for Button {
    fn get_widget(&self) -> gtk::Widget {
        self.inner.clone().upcast::<gtk::Widget>()
    }
}
