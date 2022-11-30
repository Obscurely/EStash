use super::core::LoginError;
use crate::vault;
use crate::{encrypter::key_encrypt::KeyEncrypt, utils::db};
use fltk::input;
use fltk::prelude::*;
use fltk::text;
use fltk::window::DoubleWindow;
use std::sync::{Arc, Mutex};

pub fn login_button_callback(
    input_user_arc: Arc<Mutex<input::Input>>,
    input_pass_arc: Arc<Mutex<input::Input>>,
    text_status: &mut text::TextDisplay,
    wind_clone: &mut DoubleWindow,
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

    // parse some stuff
    let vault_name = input_user.value();
    let password = input_pass.value();

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
    let mut key_encrypt = KeyEncrypt::new();

    // super::core::create_vault(&vault_name, &password, &mut estashdb, &mut argon, &mut ecies, &mut key_encrypt, is_windows);
    let vault =
        match super::core::login_vault(&vault_name, &password, &mut estashdb, &mut key_encrypt) {
            Ok(v) => {
                let mut text_status_buf = fltk::text::TextBuffer::default();
                text_status_buf.set_text("Status: Successfully logging in!");
                text_status.set_buffer(text_status_buf);
                v
            }
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
}
