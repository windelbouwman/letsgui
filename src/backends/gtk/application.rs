use super::MyError;
use super::Window;
use gio::prelude::*;
use gtk::prelude::*;

pub struct Application {
    app: gtk::Application,
}

impl Application {
    pub fn new() -> Result<Self, MyError> {
        let app = gtk::Application::new(None, Default::default()).unwrap();
        Ok(Application { app })
    }

    pub fn run(&self, window: Window) {
        // .....
        self.app.run(&[]);
    }
}

/// Entry function runs the given function to initialize windows and widgets.
/// Next up, it blocks into the IO loop.
pub fn letsgui<F>(init_func: F) -> Result<(), MyError>
where
    F: Fn() -> Result<Window, MyError> + 'static,
{
    let app = gtk::Application::new(
        Some("com.github.windelbouwman.letsgui"),
        gio::ApplicationFlags::NON_UNIQUE,
    )
    .unwrap();
    app.connect_activate(move |app| {
        let window = init_func().unwrap();
        window.show();
        window.inner.set_application(Some(app));
    });

    app.run(&[]);

    Ok(())
}
