// #![windows_subsystem = "windows"]

use letsgui::{Button, Entry, Label, ProgressBar, Slider, Win32Result, Window};

fn main() -> Win32Result {
    letsgui::letsgui(|| {
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

        let entry = Entry::new()?;
        window.add_control(Box::new(entry));

        let slider = Slider::new()?;
        window.add_control(Box::new(slider));

        Ok(window)
    })
}
