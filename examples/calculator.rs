use letsgui::{Button, Label, Win32Result, Window};

fn main() -> Win32Result {
    letsgui::letsgui(|| {
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

        Ok(window)
    })
}
