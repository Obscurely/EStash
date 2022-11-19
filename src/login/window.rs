use fltk::{
    app,
    button::Button,
    enums::{Align, Font},
    frame::Frame,
    group::Flex,
    prelude::*,
    window::Window,
};
use fltk_theme::{color_themes, ColorTheme, SchemeType, WidgetScheme};

pub fn create() -> fltk::window::DoubleWindow {
    // Create login window
    let mut wind = Window::default().with_size(500, 200).with_label("Login");
    let mut flex = Flex::default()
        .with_size(460, 160)
        .center_of_parent()
        .column();

    let mut input_user = fltk::input::Input::default();
    input_user.set_color(input_user.color().lighter());
    input_user.set_text_size(20);

    let mut input_pass = fltk::input::Input::default();
    input_pass.set_color(input_pass.color().lighter());
    input_pass.set_text_size(20);

    let mut but_login = Button::default().with_label("Login");
    but_login.set_color(but_login.color().lighter());

    // End customizing window
    flex.end();
    wind.end();

    // Window callbacks
    but_login.set_callback(move |btn| {
        let username = input_user.value();
        let password = input_pass.value();
        // will handle db later, for now just print
        println!("Usernmae: {username}\nPassword: {password}");
    });
    
    wind
}
