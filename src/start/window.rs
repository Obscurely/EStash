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
use crate::{utils::db, encrypter::{ecies::ECIES, key_encrypt::KeyEncrypt}, hasher::argon2id::Argon2id, signup};
use crate::hasher::blake3;
use crate::hasher::argon2id;
use std::sync::{Arc, Mutex};
use crate::login;
use crate::utils;

pub fn create() -> fltk::window::DoubleWindow {
    // Create start window
    let mut wind = Window::default().with_size(400, 200).with_label("Start");

    // login button
    let mut login_button = fltk::button::Button::default().with_size(170, 160).with_label("Login");
    login_button.set_pos(20, 20);

    // signup button
    let mut signup_button = fltk::button::Button::default().with_size(170, 160).with_label("Signup");
    signup_button.set_pos(210, 20);
    // signup_button.right_of(&login_button, 20);

    // End customizing window
    wind.end();

    let mut wind_clone_one = wind.clone();
    let mut wind_clone_two = wind.clone();

    // callbacks
    login_button.set_callback(move |l| {
        // hide the start window, might wanna reshow it later
        wind_clone_one.hide();

        // initialize login window
        let mut login_wind = login::window::create(utils::is_windows());
        login_wind.show();
    });

    signup_button.set_callback(move |s| {
        // hide the start window, might wanna reshow it later
        wind_clone_two.hide();

        // intialize signup window
        let mut signup_wind = signup::window::create(utils::is_windows());
        signup_wind.show();
    });

    wind
}