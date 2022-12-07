use fltk::{
    enums::Color,
    button::Button,
    frame,
    group::{self, Flex},
    prelude::*,
    window::{DoubleWindow, Window},
};
use std::sync::{Arc, Mutex};

pub fn create(
    is_windows: bool,
    start_wind: Arc<Mutex<DoubleWindow>>,
) -> fltk::window::DoubleWindow {
    //
    //  Make window | UI Part
    //

    // Create login window
    let mut wind = Window::default().with_size(1000, 500).with_label("Login");
    wind.set_xclass("estash");
    // title
    let mut title = frame::Frame::default();
    title.set_label("EStash");
    title.set_label_color(Color::from_rgb(140, 140, 140));
    // go back button
    let mut back_button = Button::default().with_size(25, 20).with_label("<");
    let back_button_arc = Arc::new(Mutex::new(back_button.clone()));
    back_button.set_label_color(Color::from_rgb(140, 140, 140));
    back_button.set_color(Color::from_rgb(43, 43, 43));

    // set window layout
    let mut flex = Flex::default();
    flex.set_type(group::FlexType::Column);
    flex.make_resizable(true);

    // vault name input
    let mut input_user = fltk::input::Input::default();
    input_user.set_color(input_user.color().lighter());
    let input_user_arc = Arc::new(Mutex::new(input_user.clone()));
    input_user.set_color(Color::from_rgb(31, 31, 31));
    input_user.set_text_color(Color::from_rgb(140, 140, 140));

    // password for vault input
    let mut input_pass = fltk::input::SecretInput::default();
    input_pass.set_color(input_pass.color().lighter());
    let input_pass_arc = Arc::new(Mutex::new(input_pass.clone()));
    input_pass.set_color(Color::from_rgb(31, 31, 31));
    input_pass.set_text_color(Color::from_rgb(140, 140, 140));

    // login button
    let mut but_login = Button::default().with_label("Login");
    but_login.set_color(but_login.color().lighter());
    let but_login_arc = Arc::new(Mutex::new(but_login.clone()));
    but_login.set_label_color(Color::from_rgb(140, 140, 140));
    but_login.set_color(Color::from_rgb(51, 51, 51));

    // the status of the operation
    let mut text_status = fltk::text::TextDisplay::default();
    text_status.set_color(Color::from_rgb(23, 23, 23));
    text_status.set_text_color(Color::from_rgb(140, 140, 140));
    let mut text_status_buf = fltk::text::TextBuffer::default();
    text_status_buf.set_text("Status: Nothing");
    text_status.set_buffer(text_status_buf);
    let text_status_arc = Arc::new(Mutex::new(text_status.clone()));

    // End customizing window
    wind.end();
    wind.make_resizable(true);

    // set window position and size same as start window
    match start_wind.lock() {
        Ok(w) => {
            wind.set_pos(w.x(), w.y());
            wind.set_size(w.width(), w.height());
        }
        Err(err) => {
            eprintln!("ERROR: Failed to set signup window size and position, poison error!\n{err}");
        }
    };

    // clone the window so we can move it in the callback
    let mut wind_clone = wind.clone();
    let mut wind_clone2 = wind.clone();

    //
    //  Window callbacks
    //

    // clone the needed arc references
    let input_user_arc_clone = input_user_arc.clone();
    let input_pass_arc_clone = input_pass_arc.clone();
    let but_login_arc_clone = but_login_arc.clone();
    let text_status_arc_clone = text_status_arc.clone();
    let back_button_arc_clone = back_button_arc.clone();
    // resize callback
    wind.resize_callback(move |_, _, _, w, h| {
        super::callbacks::resize_callback(
            w,
            h,
            &mut title,
            back_button_arc_clone.clone(),
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

    // set back button callback
    back_button.set_callback(move |_| {
        wind_clone2.hide();
        match start_wind.lock() {
            Ok(mut win) => {
                win.set_pos(wind_clone2.x(), wind_clone2.y());
                win.set_size(wind_clone2.width(), wind_clone2.height());
                win.show();
            }
            Err(err) => {
                eprintln!("ERROR: Failed to get valune under start window ARC!\n{err}");
            }
        }
    });

    wind
}
