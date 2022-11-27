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
use crate::{utils::db, encrypter::{ecies::ECIES, key_encrypt::KeyEncrypt}, hasher::argon2id::Argon2id};
use crate::hasher::blake3;
use crate::hasher::argon2id;
use std::sync::{Arc, Mutex};
use crate::vault;

pub fn create(is_windows: bool) -> fltk::window::DoubleWindow {
    // Create login window
    let mut wind = Window::default().with_size(710, 200).with_label("Login");
    let mut flex = Flex::default()
        .with_size(500, 160)
        .center_of_parent()
        .column();

    let mut input_user = fltk::input::Input::default();
    input_user.set_color(input_user.color().lighter());
    input_user.set_text_size(20);
    input_user.set_label("Vault Name ");
    input_user.set_label_font(Font::ScreenBold);

    let mut input_pass = fltk::input::Input::default();
    input_pass.set_color(input_pass.color().lighter());
    input_pass.set_text_size(20);
    input_pass.set_label("Password ");
    input_pass.set_label_font(Font::ScreenBold);

    let mut but_login = Button::default().with_label("Login");
    but_login.set_color(but_login.color().lighter());
    but_login.set_label_font(Font::ScreenBold);

    let mut text_status = fltk::text::TextDisplay::default();
    text_status.set_color(text_status.color().lighter());
    text_status.set_text_size(15);
    text_status.set_text_font(Font::ScreenBold);
    let mut text_status_buf = fltk::text::TextBuffer::default();
    text_status_buf.set_text("Status: Nothing");
    text_status.set_buffer(text_status_buf);

    // End customizing window
    flex.end();
    wind.end();

    let mut wind_clone = wind.clone();

    // Window callbacks
    but_login.set_callback(move |btn| {
        // parse some stuff
        let vault_name = input_user.value();
        let password = input_pass.value();

        // load necessary databases
        let mut estashdb = db::EstashDb::new().unwrap();

        // create necessary objects
        let mut argon = Argon2id::new();
        let mut ecies = ECIES::new();
        let mut key_encrypt = KeyEncrypt::new();

        // super::core::create_vault(&vault_name, &password, &mut estashdb, &mut argon, &mut ecies, &mut key_encrypt, is_windows);
        let vault = super::core::login_vault(&vault_name, &password, &mut estashdb, &mut argon, &mut ecies, &mut key_encrypt, is_windows);

        // open vault window
        wind_clone.hide();
        let mut vault_wind = vault::window::create(is_windows, vault);
        vault_wind.show();
    });
    
    wind
}
