use fltk::{
    button::Button,
    enums::Font,
    frame,
    group::{self, Flex},
    prelude::*,
    window::Window,
};
use std::sync::{Arc, Mutex};

pub fn create(is_windows: bool) -> fltk::window::DoubleWindow {
    //
    //  Make window | UI Part
    //

    // Create login window
    let mut wind = Window::default().with_size(710, 200).with_label("Login");
    let mut title = frame::Frame::default();
    title.set_label("EStash");

    // set window layout
    let mut flex = Flex::default();
    flex.set_type(group::FlexType::Column);
    flex.make_resizable(true);

    // vault name input
    let mut input_user = fltk::input::Input::default();
    input_user.set_color(input_user.color().lighter());
    let input_user_arc = Arc::new(Mutex::new(input_user.clone()));

    // password for vault input
    let mut input_pass = fltk::input::Input::default();
    input_pass.set_color(input_pass.color().lighter());
    let input_pass_arc = Arc::new(Mutex::new(input_pass.clone()));

    // login button
    let mut but_login = Button::default().with_label("Login");
    but_login.set_color(but_login.color().lighter());
    but_login.set_label_font(Font::ScreenBold);
    let but_login_arc = Arc::new(Mutex::new(but_login.clone()));

    // the status of the operation
    let mut text_status = fltk::text::TextDisplay::default();
    text_status.set_color(text_status.color().lighter());
    text_status.set_text_font(Font::ScreenBold);
    let mut text_status_buf = fltk::text::TextBuffer::default();
    text_status_buf.set_text("Status: Nothing");
    text_status.set_buffer(text_status_buf);
    let text_status_arc = Arc::new(Mutex::new(text_status.clone()));

    // End customizing window
    wind.end();
    wind.make_resizable(true);

    // clone the window so we can move it in the callback
    let mut wind_clone = wind.clone();

    //
    //  Window callbacks
    //

    // clone the needed arc references
    let input_user_arc_clone = input_user_arc.clone();
    let input_pass_arc_clone = input_pass_arc.clone();
    let but_login_arc_clone = but_login_arc.clone();
    let text_status_arc_clone = text_status_arc.clone();
    // resize callback
    wind.resize_callback(move |_, _, _, w, h| {
        super::callbacks::resize_callback(
            w,
            h,
            &mut title,
            input_user_arc_clone.clone(),
            input_pass_arc_clone.clone(),
            but_login_arc_clone.clone(),
            text_status_arc_clone.clone(),
            &mut flex,
        )
    });

    // clone the needed arc references
    let input_user_arc_clone = input_user_arc.clone();
    let input_pass_arc_clone = input_pass_arc.clone();
    let text_status_arc_clone = text_status_arc.clone();
    // login button callback
    but_login.set_callback(move |_| {
        super::callbacks::login_button_callback(
            input_user_arc_clone.clone(),
            input_pass_arc_clone.clone(),
            text_status_arc_clone.clone(),
            &mut wind_clone,
            is_windows,
        )
    });

    wind
}
