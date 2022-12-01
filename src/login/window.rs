use fltk::{
    button::Button,
    enums::Font,
    frame,
    group::{self, Flex},
    prelude::*,
    window::Window,
};
use fltk_grid::Grid;
use std::f32;
use std::sync::{Arc, Mutex};

pub fn create(is_windows: bool) -> fltk::window::DoubleWindow {
    // Create login window
    let mut wind = Window::default().with_size(710, 200).with_label("Login");
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

    let mut but_login = Button::default().with_label("Login");
    but_login.set_color(but_login.color().lighter());
    but_login.set_label_font(Font::ScreenBold);
    let but_login_arc = Arc::new(Mutex::new(but_login.clone()));

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

    let mut wind_clone = wind.clone();

    let input_user_arc_clone = input_user_arc.clone();
    let input_pass_arc_clone = input_pass_arc.clone();
    let but_login_arc_clone = but_login_arc.clone();
    let text_status_arc_clone = text_status_arc.clone();
    // resize callback
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
        }

        match input_pass_arc_clone.lock() {
            Ok(mut o) => {
                o.set_text_size(font_size);
            }
            Err(err) => {
                eprintln!("ERROR: There was an error changing input_pass text size, arc poison error!\n{err}");
            }
        }

        match but_login_arc_clone.lock() {
            Ok(mut o) => {
                o.set_label_size(font_size);
            }
            Err(err) => {
                eprintln!("ERROR: There was an error changing but_login text size, arc poison error!\n{err}");
            }
        }

        match text_status_arc_clone.lock() {
            Ok(mut o) => {
                o.set_text_size(font_size / 3);
            }
            Err(err) => {
                eprintln!("ERROR: There was an error changing text_status text size, arc poison error!\n{err}");
            }
        }

        flex.resize(w_center / 2, h_center / 2, w_center, h_center);
    });

    let input_user_arc_clone = input_user_arc.clone();
    let input_pass_arc_clone = input_pass_arc.clone();
    let text_status_arc_clone = text_status_arc.clone();
    // Window callbacks
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
