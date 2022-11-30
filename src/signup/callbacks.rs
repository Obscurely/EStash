use super::core::SingupError;
use crate::{
    encrypter::{ecies::ECIES, key_encrypt::KeyEncrypt},
    utils::db,
};
use fltk::{input, prelude::*, text::TextDisplay};
use std::sync::{Arc, Mutex};

pub fn singup_button_callback(
    input_user_arc: Arc<Mutex<input::Input>>,
    input_pass_arc: Arc<Mutex<input::Input>>,
    input_pass_again_arc: Arc<Mutex<input::Input>>,
    text_status: &mut TextDisplay,
    is_windows: bool,
) {
    // get values behind arc
    let input_user = match input_user_arc.lock() {
        Ok(object) => object,
        Err(err) => {
            eprintln!("ERROR: Failed to get value under input_user ARC!\n{err}");
            let mut text_status_buf = fltk::text::TextBuffer::default();
            text_status_buf.set_text("Status: There was a Poison Error, try to restart!");
            text_status.set_buffer(text_status_buf);
            return;
        }
    };
    let input_pass = match input_pass_arc.lock() {
        Ok(object) => object,
        Err(err) => {
            eprintln!("ERROR: Failed to get value under input_pass ARC!\n{err}");
            let mut text_status_buf = fltk::text::TextBuffer::default();
            text_status_buf.set_text("Status: There was a Poison Error, try to restart!");
            text_status.set_buffer(text_status_buf);
            return;
        }
    };
    let input_pass_again = match input_pass_again_arc.lock() {
        Ok(object) => object,
        Err(err) => {
            eprintln!("ERROR: Failed to get value under input_pass_again ARC!\n{err}");
            let mut text_status_buf = fltk::text::TextBuffer::default();
            text_status_buf.set_text("Status: There was a Poison Error, try to restart!");
            text_status.set_buffer(text_status_buf);
            return;
        }
    };

    // parse some stuff
    let vault_name = input_user.value();
    let password = input_pass.value();
    let password_again = input_pass_again.value();

    // load necessary databases
    let mut estashdb = match db::EstashDb::new() {
        Ok(db) => db,
        Err(err) => {
            eprintln!(
                "ERROR: There was an error reading the db containing the list with vaults!\n{err}"
            );
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
            Err(SingupError::FailedToStorePrivateKey(_)) => {
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
}
