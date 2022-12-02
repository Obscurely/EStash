use crate::login;
use crate::signup;
use crate::utils;
use fltk::frame;
use fltk::group;
use fltk::group::Flex;
use fltk::{prelude::*, window::Window};
use std::sync::{Arc, Mutex};

pub fn create() -> fltk::window::DoubleWindow {
    //
    //  Make window | UI Part
    //

    // Create start window
    let mut wind = Window::default().with_size(400, 200).with_label("Start");
    wind.set_xclass("estash");
    // app title
    let mut title = frame::Frame::default();
    title.set_label("EStash");

    // flex
    let mut flex = Flex::default();
    flex.set_type(group::FlexType::Row);
    flex.make_resizable(true);

    // login button
    let mut login_button = fltk::button::Button::default().with_label("Login");
    let login_button_arc = Arc::new(Mutex::new(login_button.clone()));

    // signup button
    let mut signup_button = fltk::button::Button::default().with_label("Signup");
    let signup_button_arc = Arc::new(Mutex::new(signup_button.clone()));

    // End customizing window
    wind.end();
    wind.make_resizable(true);

    // clone the wind so we can move it inside the callbacks
    let mut wind_clone_one = wind.clone();
    let mut wind_clone_two = wind.clone();

    //
    //  Window callbacks
    //

    // resize callback
    wind.resize_callback(move |_, _, _, w, h| {
        let w_center = w / 2 ;
        let h_center = h / 2 ;
        let font_size = (f32::sqrt(w as f32 * h as f32) / 20.0).floor() as i32;

        // configure title
        title.set_label_size(font_size * 2);
        title.set_pos((w / 2) - (font_size / 24), font_size);

        match login_button_arc.lock() {
            Ok(mut o) => {
                o.set_label_size(font_size);
            }
            Err(err) => {
                eprintln!("ERROR: There was an error changing login_button text size, arc poison error!\n{err}");
            }
        };

        match signup_button_arc.lock() {
            Ok(mut o) => {
                o.set_label_size(font_size);
            }
            Err(err) => {
                eprintln!("ERROR: There was an error changing signup_button text size, arc poison error!\n{err}");
            }
        }

        flex.resize(w_center / 2, h_center / 2, w_center, h_center);
    });

    // callbacks
    login_button.set_callback(move |_| {
        // hide the start window, might wanna reshow it later
        wind_clone_one.hide();

        // initialize login window
        let mut login_wind = login::window::create(utils::is_windows());
        login_wind.show();
    });

    signup_button.set_callback(move |_| {
        // hide the start window, might wanna reshow it later
        wind_clone_two.hide();

        // intialize signup window
        let mut signup_wind = signup::window::create(utils::is_windows());
        signup_wind.show();
    });

    wind
}
