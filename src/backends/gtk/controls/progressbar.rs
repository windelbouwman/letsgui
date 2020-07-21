use super::{GtkControl, MyError};
use gtk::prelude::*;

pub struct ProgressBar {
    inner: gtk::ProgressBar,
}

impl ProgressBar {
    pub fn new() -> Result<Self, MyError> {
        let inner = gtk::ProgressBar::new();
        Ok(ProgressBar { inner })
    }

    pub fn set_value(&self, value: usize) -> Result<(), MyError> {
        self.inner.set_fraction(value as f64 / 100.0);
        Ok(())
    }
}

impl GtkControl for ProgressBar {
    fn get_widget(&self) -> gtk::Widget {
        self.inner.clone().upcast::<gtk::Widget>()
    }
}
