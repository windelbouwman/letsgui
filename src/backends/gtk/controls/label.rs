use super::GtkControl;
use super::MyError;
use gtk::prelude::*;

pub struct Label {
    inner: gtk::Label,
}

impl Label {
    pub fn new(text: &str) -> Result<Self, MyError> {
        let inner = gtk::Label::new(Some(text));
        Ok(Label { inner })
    }
}

impl GtkControl for Label {
    fn get_widget(&self) -> gtk::Widget {
        self.inner.clone().upcast::<gtk::Widget>()
    }
}
