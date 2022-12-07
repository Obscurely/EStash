use fltk::{
    button::Button,
    frame,
    group::{self, Flex},
    prelude::*,
    window::{DoubleWindow, Window}, enums::Color,
};
use std::sync::{Arc, Mutex};

pub fn create(
    is_windows: bool,
    start_wind: Arc<Mutex<DoubleWindow>>,
) -> fltk::window::DoubleWindow {
    //
    //  Make window | UI Part
    //

    // Create signup window
    let mut wind = Window::default().with_size(1000, 500).with_label("Singup");
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

    // window layout
    let mut flex = Flex::default();
    flex.set_type(group::FlexType::Column);
    flex.make_resizable(true);

    // vault name input
    let mut input_user = fltk::input::Input::default();
    input_user.set_color(input_user.color().lighter());
    let input_user_arc = Arc::new(Mutex::new(input_user.clone()));
    input_user.set_color(Color::from_rgb(31, 31, 31));
    input_user.set_text_color(Color::from_rgb(140, 140, 140));

    // password input
    let mut input_pass = fltk::input::SecretInput::default();
    input_pass.set_color(input_pass.color().lighter());
    let input_pass_arc = Arc::new(Mutex::new(input_pass.clone()));
    input_pass.set_color(Color::from_rgb(31, 31, 31));
    input_pass.set_text_color(Color::from_rgb(140, 140, 140));

    // password, again input
    let mut input_pass_again = fltk::input::SecretInput::default();
    input_pass_again.set_color(input_pass_again.color().lighter());
    let input_pass_again_arc = Arc::new(Mutex::new(input_pass_again.clone()));
    input_pass_again.set_color(Color::from_rgb(31, 31, 31));
    input_pass_again.set_text_color(Color::from_rgb(140, 140, 140));

    // signup button
    let mut but_signup = Button::default().with_label("Signup");
    but_signup.set_color(but_signup.color().lighter());
    let but_signup_arc = Arc::new(Mutex::new(but_signup.clone()));
    but_signup.set_label_color(Color::from_rgb(140, 140, 140));
    but_signup.set_color(Color::from_rgb(51, 51, 51));

    // the status of the operation
    let mut text_status = fltk::text::TextDisplay::default();
    text_status.set_color(Color::from_rgb(23, 23, 23));
    text_status.set_text_color(Color::from_rgb(140, 140, 140));
    let mut text_status_buf = fltk::text::TextBuffer::default();
    text_status_buf.set_text("Status: Nothing");
    text_status.set_buffer(text_status_buf);
    let text_status_arc = Arc::new(Mutex::new(text_status.clone()));

    // End customizing window
    flex.end();
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

    //
    //  Window callbacks
    //

    // clone the needed arc references
    let input_user_arc_clone = input_user_arc.clone();
    let input_pass_arc_clone = input_pass_arc.clone();
    let input_pass_again_arc_clone = input_pass_again_arc.clone();
    let text_status_arc_clone = text_status_arc.clone();
    let but_signup_arc_clone = but_signup_arc.clone();
    let back_button_arc_clone = back_button_arc.clone();
    // window callback
    wind.resize_callback(move |_, _, _, w, h| {
        super::callbacks::window_callback(
            w,
            h,
            &mut title,
            back_button_arc_clone.clone(),
            input_user_arc_clone.clone(),
            input_pass_arc_clone.clone(),
            input_pass_again_arc_clone.clone(),
            but_signup_arc_clone.clone(),
            text_status_arc_clone.clone(),
            &mut flex,
        )
    });

    // clone the needed arc references
    let input_user_arc_clone = input_user_arc.clone();
    let input_pass_arc_clone = input_pass_arc.clone();
    let input_pass_again_arc_clone = input_pass_again_arc.clone();
    let text_status_arc_clone = Arc::new(Mutex::new(text_status.clone()));
    // signup button callback
    but_signup.set_callback(move |_| {
        super::callbacks::singup_button_callback(
            input_user_arc_clone.clone(),
            input_pass_arc_clone.clone(),
            input_pass_again_arc_clone.clone(),
            text_status_arc_clone.clone(),
            is_windows,
        )
    });

    // clone the needed objects
    let mut wind_clone = wind.clone();
    // set back button callback
    back_button.set_callback(move |_| {
        wind_clone.hide();
        match start_wind.lock() {
            Ok(mut win) => {
                win.set_pos(wind_clone.x(), wind_clone.y());
                win.set_size(wind_clone.width(), wind_clone.height());
                win.show();
            }
            Err(err) => {
                eprintln!("ERROR: Failed to get valune under start window ARC!\n{err}");
            }
        }
    });

    wind
}
