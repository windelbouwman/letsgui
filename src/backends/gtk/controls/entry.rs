use super::GtkControl;
use super::MyError;
use gtk::prelude::*;

pub struct Entry {
    inner: gtk::Entry,
}

impl Entry {
    pub fn new() -> Result<Self, MyError> {
        let inner = gtk::Entry::new();
        Ok(Entry { inner })
    }
}

impl GtkControl for Entry {
    fn get_widget(&self) -> gtk::Widget {
        self.inner.clone().upcast::<gtk::Widget>()
    }
}
