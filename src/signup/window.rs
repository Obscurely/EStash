use fltk::{button::Button, enums::Font, group::{Flex, self}, prelude::*, window::Window, frame};
use std::sync::{Arc, Mutex};

pub fn create(is_windows: bool) -> fltk::window::DoubleWindow {
    // Create signup window
    let mut wind = Window::default().with_size(710, 200).with_label("Singup");

    // title
    let mut title = frame::Frame::default();
    title.set_label("EStash");

    let mut flex = Flex::default();
    flex.set_type(group::FlexType::Column);
    flex.make_resizable(true);

    let mut input_user = fltk::input::Input::default();
    input_user.set_color(input_user.color().lighter());
    let input_user_arc = Arc::new(Mutex::new(input_user.clone()));

    let mut input_pass = fltk::input::Input::default();
    input_pass.set_color(input_pass.color().lighter());
    let input_pass_arc = Arc::new(Mutex::new(input_pass.clone()));

    let mut input_pass_again = fltk::input::Input::default();
    input_pass_again.set_color(input_pass_again.color().lighter());
    let input_pass_again_arc = Arc::new(Mutex::new(input_pass_again.clone()));

    let mut but_signup = Button::default().with_label("Signup");
    but_signup.set_color(but_signup.color().lighter());
    but_signup.set_label_font(Font::ScreenBold);
    let but_signup_arc = Arc::new(Mutex::new(but_signup.clone()));

    let mut text_status = fltk::text::TextDisplay::default();
    text_status.set_color(text_status.color().lighter());
    text_status.set_text_font(Font::ScreenBold);
    let mut text_status_buf = fltk::text::TextBuffer::default();
    text_status_buf.set_text("Status: Nothing");
    text_status.set_buffer(text_status_buf);
    let text_status_arc = Arc::new(Mutex::new(text_status.clone()));

    // End customizing window
    flex.end();
    wind.end();
    wind.make_resizable(true);

    let input_user_arc_clone = input_user_arc.clone();
    let input_pass_arc_clone = input_pass_arc.clone();
    let input_pass_again_arc_clone = input_pass_again_arc.clone();
    let text_status_arc_clone = text_status_arc.clone();
    let but_signup_arc_clone = but_signup_arc.clone();
    wind.resize_callback(move |_, _, _, w, h| {
        let w_center = w / 2 ;
        let h_center = h / 2 ;
        let font_size = (f32::sqrt(w as f32 * h as f32) / 20.0).floor() as i32;

        title.set_label_size(font_size * 2);
        title.set_pos((w / 2) - (font_size / 24), font_size);

        match input_user_arc_clone.lock() {
            Ok(mut o) => {
                o.set_text_size(font_size);
            }
            Err(err) => {
                eprintln!("ERROR: There was an error changing input_user text size, arc poison error!\n{err}");
            }
        };

        match input_pass_arc_clone.lock() {
            Ok(mut o) => {
                o.set_text_size(font_size);
            }
            Err(err) => {
                eprintln!("ERROR: There was an error changing input_pass text size, arc poison error!\n{err}");
            }
        };
        
        match input_pass_again_arc_clone.lock() {
            Ok(mut o) => {
                o.set_text_size(font_size);
            },
            Err(err) => {
                eprintln!("ERROR: There was an error changing input_pass_again text size, arc poison error!\n{err}");
            },
        };

        match but_signup_arc_clone.lock() {
            Ok(mut o) => {
                o.set_label_size(font_size);
            }
            Err(err) => {
                eprintln!("ERROR: There was an error changing but_signup size, arc poison error!\n{err}");
            }
        }

        match text_status_arc_clone.lock() {
            Ok(mut o) => {
                o.set_text_size(font_size / 3);
            }
            Err(err) => {
                eprintln!("ERROR: There was an error changing text_status text size, arc poison error!\n{err}");
            }
        };

        flex.resize(w_center / 2, h_center / 2, w_center, h_center);
    });

    let input_user_arc_clone = input_user_arc.clone();
    let input_pass_arc_clone = input_pass_arc.clone();
    let input_pass_again_arc_clone = input_pass_again_arc.clone();
    let text_status_arc_clone = Arc::new(Mutex::new(text_status.clone()));
    // Window callbacks
    but_signup.set_callback(move |_| {
        super::callbacks::singup_button_callback(
            input_user_arc_clone.clone(),
            input_pass_arc_clone.clone(),
            input_pass_again_arc_clone.clone(),
            text_status_arc_clone.clone(),
            is_windows,
        )
    });

    wind
}
