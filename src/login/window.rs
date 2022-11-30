use crate::vault;
use crate::{
    encrypter::key_encrypt::KeyEncrypt,
    utils::db,
};
use fltk::{
    button::Button,
    enums::Font,
    group::Flex,
    prelude::*,
    window::Window,
};

use super::core::LoginError;

pub fn create(is_windows: bool) -> fltk::window::DoubleWindow {
    // Create login window
    let wind = Window::default().with_size(710, 200).with_label("Login");
    let flex = Flex::default()
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
    but_login.set_callback(move |_| {
        // parse some stuff
        let vault_name = input_user.value();
        let password = input_pass.value();

        // load necessary databases
        let mut estashdb = match db::EstashDb::new() {
            Ok(db) => db,
            Err(err) => {
                eprintln!("ERROR: There was an error reading the db containing the list with vaults!\n{err}");
                let mut text_status_buf = fltk::text::TextBuffer::default();
                text_status_buf.set_text("Status: Failed to load db with vaults!");
                text_status.set_buffer(text_status_buf);
                return;
            }
        };

        // create necessary objects
        let mut key_encrypt = KeyEncrypt::new();

        // super::core::create_vault(&vault_name, &password, &mut estashdb, &mut argon, &mut ecies, &mut key_encrypt, is_windows);
        let vault = match super::core::login_vault(
            &vault_name,
            &password,
            &mut estashdb,
            &mut key_encrypt,
        ) {
            Ok(v) => {
                let mut text_status_buf = fltk::text::TextBuffer::default();
                text_status_buf.set_text("Status: Successfully logging in!");
                text_status.set_buffer(text_status_buf);
                v
            },
            Err(LoginError::CorruptedVault(_)) => {
                let mut text_status_buf = fltk::text::TextBuffer::default();
                text_status_buf.set_text("Status: This vault is corrupted!");
                text_status.set_buffer(text_status_buf);
                return;
            }
            Err(LoginError::WrongCredentials(_)) => {
                let mut text_status_buf = fltk::text::TextBuffer::default();
                text_status_buf.set_text("Status: The input credentials are wrong!");
                text_status.set_buffer(text_status_buf);
                return; 
            }
            Err(LoginError::CorruptedVaultsDb(_)) => {
                let mut text_status_buf = fltk::text::TextBuffer::default();
                text_status_buf.set_text("Status: The vaults db is corrupted!");
                text_status.set_buffer(text_status_buf);
                return;
            }
            Err(LoginError::CorruptedPubKeyDb(_)) => {
                let mut text_status_buf = fltk::text::TextBuffer::default();
                text_status_buf.set_text("Status: The pub key db is corrupted!");
                text_status.set_buffer(text_status_buf);
                return;
            }
            Err(LoginError::FailedToAccessVaultsDb(_)) => {
                let mut text_status_buf = fltk::text::TextBuffer::default();
                text_status_buf.set_text("Status: Failed to access the vaults db!");
                text_status.set_buffer(text_status_buf);
                return;
            }
            Err(LoginError::FailedToAccessPubKeyDb(_)) => {
                let mut text_status_buf = fltk::text::TextBuffer::default();
                text_status_buf.set_text("Status: Failed to access the pub key db!");
                text_status.set_buffer(text_status_buf);
                return;
            }
            Err(LoginError::FailedToAccessPrivKeyDb(_)) => {
                let mut text_status_buf = fltk::text::TextBuffer::default();
                text_status_buf.set_text("Status: Failed to access the priv key db!");
                text_status.set_buffer(text_status_buf);
                return;
            }
        };

        // open vault window
        wind_clone.hide();
        let mut vault_wind = vault::window::create(is_windows, vault);
        vault_wind.show();
    });

    wind
}
