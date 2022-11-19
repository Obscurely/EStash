mod utils;
mod hasher;
mod encrypter;
mod login;
mod signup;
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
use hasher::argon2id::Argon2id;
use encrypter::ecies::ECIES;
use encrypter::key_encrypt::KeyEncrypt;
use std::sync::{Arc, Mutex};

fn main() {
    // load necessary databases
    let mut estashdb = utils::db::EstashDb::new().unwrap();

    // create necessary objects
    let mut argon = Argon2id::new();
    let mut ecies = ECIES::new();
    let mut key_encrypt = KeyEncrypt::new();

    // Configure app and theme it
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let theme = ColorTheme::new(color_themes::BLACK_THEME);
    theme.apply();
    let widget_scheme = WidgetScheme::new(SchemeType::Clean);
    widget_scheme.apply();

    // Create signup window
    let mut signup_wind = signup::window::create(&mut estashdb, &mut argon, &mut ecies, &mut key_encrypt, utils::is_windows()); 
    signup_wind.show();

    // Start the app
    app.run().unwrap();
}
