use super::{Button, GtkControl, MyError};
use gtk::prelude::*;
// {ContainerExt, WidgetExt};

pub struct Window {
    pub inner: gtk::Window,
    bx: gtk::Box,
}

impl Window {
    pub fn new(title: &str) -> Result<Self, MyError> {
        let inner = gtk::Window::new(gtk::WindowType::Toplevel);
        let bx = gtk::Box::new(gtk::Orientation::Vertical, 0);
        inner.add(&bx);
        Ok(Window { inner, bx })
    }

    pub fn add_control(&self, control: Box<dyn GtkControl>) {
        self.bx.add(&control.get_widget());
    }

    pub fn show(&self) {
        self.inner.show_all();
    }
}
