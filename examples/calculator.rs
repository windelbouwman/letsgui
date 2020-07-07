use letsgui::win32::{Button, Label, Win32Result, Window};

fn main() -> Win32Result {
    letsgui::win32::initialize()?;

    println!("MOI");

    let window = Window::new("Calculator")?;

    let label = Label::new("")?;
    let button_zero = Button::new("0")?;
    let button_one = Button::new("1")?;
    let button_equals = Button::new("=")?;
    let button_add = Button::new("+")?;
    window.add_control(Box::new(label));
    window.add_control(Box::new(button_zero));
    window.add_control(Box::new(button_one));
    window.add_control(Box::new(button_equals));
    window.add_control(Box::new(button_add));

    // window.show();

    letsgui::win32::run_event_loop(window);

    Ok(())
}
