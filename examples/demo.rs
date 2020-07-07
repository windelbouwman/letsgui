// #![windows_subsystem = "windows"]

use letsgui;
use letsgui::win32::{Button, Label, ProgressBar, Slider, Win32Result, Window};

fn main() {
    fubar().unwrap();
}

pub fn fubar() -> Win32Result {
    println!("ARRR1");
    letsgui::win32::initialize()?;

    println!("ARRR2");

    let window = Window::new("FUBAR")?;
    // let vbox = create_container(window.hwnd, h_instance)?;

    let button = Button::new("OK")?;
    // button.on_clicked(|b| {
    //     println!("Clicked!");
    // });
    window.add_control(Box::new(button));

    let label1 = Label::new("Das Mooi!")?;
    window.add_control(Box::new(label1));
    let pb = ProgressBar::new()?;
    pb.set_value(25)?;
    window.add_control(Box::new(pb));
    let label2 = Label::new("Man!")?;
    window.add_control(Box::new(label2));

    let slider = Slider::new()?;
    window.add_control(Box::new(slider));

    // Step 3: run event loop:
    letsgui::win32::run_event_loop(window);

    Ok(())
}
