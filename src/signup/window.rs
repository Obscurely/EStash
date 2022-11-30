use fltk::{button::Button, enums::Font, group::Flex, prelude::*, window::Window};
use std::sync::{Arc, Mutex};

pub fn create(is_windows: bool) -> fltk::window::DoubleWindow {
    // Create signup window
    let wind = Window::default().with_size(710, 200).with_label("Singup");
    let flex = Flex::default()
        .with_size(500, 160)
        .center_of_parent()
        .column();

    let mut input_user = fltk::input::Input::default();
    input_user.set_color(input_user.color().lighter());
    input_user.set_text_size(20);
    input_user.set_label("Vault Name ");
    input_user.set_label_font(Font::ScreenBold);
    let input_user_arc = Arc::new(Mutex::new(input_user.clone()));

    let mut input_pass = fltk::input::Input::default();
    input_pass.set_color(input_pass.color().lighter());
    input_pass.set_text_size(20);
    input_pass.set_label("Password ");
    input_pass.set_label_font(Font::ScreenBold);
    let input_pass_arc = Arc::new(Mutex::new(input_pass.clone()));

    let mut input_pass_again = fltk::input::Input::default();
    input_pass_again.set_color(input_pass_again.color().lighter());
    input_pass_again.set_text_size(18);
    input_pass_again.set_label("Repeat  ");
    input_pass_again.set_label_font(Font::ScreenBold);
    let input_pass_again_arc = Arc::new(Mutex::new(input_pass_again.clone()));

    let mut but_signup = Button::default().with_label("Signup");
    but_signup.set_color(but_signup.color().lighter());
    but_signup.set_label_font(Font::ScreenBold);

    let mut text_status = fltk::text::TextDisplay::default();
    text_status.set_color(text_status.color().lighter());
    text_status.set_text_size(15);
    text_status.set_text_font(Font::ScreenBold);
    let mut text_status_buf = fltk::text::TextBuffer::default();
    text_status_buf.set_text("Status: Nothing");
    text_status.set_buffer(text_status_buf);

    // End customizing window
    flex.end();
    wind.end();

    // Window callbacks
    but_signup.set_callback(move |_| {
        super::callbacks::singup_button_callback(
            input_user_arc.clone(),
            input_pass_arc.clone(),
            input_pass_again_arc.clone(),
            &mut text_status,
            is_windows,
        )
    });

    wind
}
