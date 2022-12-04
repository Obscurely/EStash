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

    // Create parent window
    let mut main_wind = Window::default().with_size(1000, 500);
    main_wind.set_xclass("estash");
    let main_wind_arc = Arc::new(Mutex::new(main_wind.clone()));
    // Create start window
    let mut wind = Window::default().with_size(1000, 500).with_label("Start");
    wind.set_xclass("estash");
    let wind_arc = Arc::new(Mutex::new(wind.clone()));
    

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

    // Create login and singup window
    let mut login_wind = login::window::create(utils::is_windows(), wind_arc.clone(), main_wind_arc.clone());
    login_wind.hide();
    login_wind.end();
    let mut signup_wind = signup::window::create(utils::is_windows(), wind_arc.clone());
    signup_wind.hide();
    signup_wind.end();

    // End main_wind
    main_wind.end();
    main_wind.make_resizable(true);

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

    // clone needed arc refs
    let wind_arc_clone = wind_arc.clone();
    // callbacks
    login_button.set_callback(move |_| {
        // hide start window
        match wind_arc_clone.lock() {
            Ok(mut w) => {
                w.hide();
            }
            Err(err) => {
                eprintln!("ERROR: Failed to get value under wind_arc ARC! Quitting program, a restart is better!\n{err}");
            }
        };
        // show the login wind        
        login_wind.show();
    });

    // clone needed arc refs
    let wind_arc_clone = wind_arc.clone();
    signup_button.set_callback(move |_| {
        // hide start window
        match wind_arc_clone.lock() {
            Ok(mut w) => {
                w.hide();
            }
            Err(err) => {
                eprintln!("ERROR: Failed to get value under wind_arc ARC! Quitting program, a restart is better!\n{err}");
            }
        };
        // show the singup wind
        signup_wind.show();
    });

    main_wind
}
