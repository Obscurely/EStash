use crate::{
    encrypter::{ecies::ECIES, key_encrypt::KeyEncrypt},
    utils::db,
};
use fltk::{button::Button, enums::Font, group::Flex, prelude::*, window::Window};

use super::core::SingupError;

pub fn create(is_windows: bool) -> fltk::window::DoubleWindow {
    // Create signup window
    let wind = Window::default().with_size(710, 200).with_label("Singup");
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

    let mut input_pass_again = fltk::input::Input::default();
    input_pass_again.set_color(input_pass_again.color().lighter());
    input_pass_again.set_text_size(18);
    input_pass_again.set_label("Repeat  ");
    input_pass_again.set_label_font(Font::ScreenBold);

    let mut but_signup = Button::default().with_label("Signup");
    but_signup.set_color(but_signup.color().lighter());
    but_signup.set_label_font(Font::ScreenBold);

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

    // Window callbacks
    but_signup.set_callback(move |_| {
        // parse some stuff
        let vault_name = input_user.value();
        let password = input_pass.value();
        let password_again = input_pass_again.value();

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
        let mut ecies = ECIES::new();
        let mut key_encrypt = KeyEncrypt::new();

        if password == password_again {
            match super::core::create_vault(
                &vault_name,
                &password,
                &mut estashdb,
                &mut ecies,
                &mut key_encrypt,
                is_windows,
            ) {
                Ok(_) => {
                    let mut text_status_buf = fltk::text::TextBuffer::default();
                    text_status_buf.set_text("Status: Successfully created account!");
                    text_status.set_buffer(text_status_buf);
                }
                Err(SingupError::FailedToStorePublicKey(_)) => {
                    let mut text_status_buf = fltk::text::TextBuffer::default();
                    text_status_buf.set_text("Status: Failed to store public key!");
                    text_status.set_buffer(text_status_buf);
                }
                Err(SingupError::FailedToStoreCredentials(_)) => {
                    let mut text_status_buf = fltk::text::TextBuffer::default();
                    text_status_buf.set_text("Status: Failed to store credentials!");
                    text_status.set_buffer(text_status_buf);
                }
                Err(SingupError::UnknownError(_)) => {
                    let mut text_status_buf = fltk::text::TextBuffer::default();
                    text_status_buf.set_text("Status: An unknown error occurred!");
                    text_status.set_buffer(text_status_buf);
                }
                Err(SingupError::FailedToAccessVaultsDb(_)) => {
                    let mut text_status_buf = fltk::text::TextBuffer::default();
                    text_status_buf.set_text("Status: Failed to access vaults db!");
                    text_status.set_buffer(text_status_buf);
                }
                Err(SingupError::CorruptedVaultsDb(_)) => {
                    let mut text_status_buf = fltk::text::TextBuffer::default();
                    text_status_buf.set_text("Status: Vaults db is corrupted!");
                    text_status.set_buffer(text_status_buf);
                }
                Err(SingupError::AlreadyExists(_)) => {
                    let mut text_status_buf = fltk::text::TextBuffer::default();
                    text_status_buf.set_text("Status: A vault with this credentials already exists!");
                    text_status.set_buffer(text_status_buf);
                }
                Err(SingupError::FailedToCreateVault(_)) => {
                    let mut text_status_buf = fltk::text::TextBuffer::default();
                    text_status_buf.set_text("Status: Failed to create the vault (the db its self)!");
                    text_status.set_buffer(text_status_buf);
                }
                Err(SingupError::FailedToStorePrivateKey(_) ) => {
                    let mut text_status_buf = fltk::text::TextBuffer::default();
                    text_status_buf.set_text("Status: Failed to store the private key!");
                    text_status.set_buffer(text_status_buf);
                }
            };
        } else {
            let mut text_status_buf = fltk::text::TextBuffer::default();
            text_status_buf.set_text("Status: Passwords don't match");
            text_status.set_buffer(text_status_buf);
        }
    });

    wind
}
