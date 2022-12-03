use super::core::LoginError;
use crate::vault;
use crate::{encrypter::key_encrypt::KeyEncrypt, utils::db};
use fltk::frame::Frame;
use fltk::group::Flex;
use fltk::prelude::*;
use fltk::text;
use fltk::window::DoubleWindow;
use fltk::{button, input};
use std::sync::{Arc, Mutex};

///
/// The callback function called when you hit the login button.
/// Takes the username and password and checks to see if there exists a vault
/// and if there is it tries logging in to it.
///
pub fn login_button_callback(
    input_user_arc: Arc<Mutex<input::Input>>,
    input_pass_arc: Arc<Mutex<input::Input>>,
    text_status_arc: Arc<Mutex<text::TextDisplay>>,
    wind_clone: &mut DoubleWindow,
    is_windows: bool,
) {
    // get values behind arc
    let mut text_status = match text_status_arc.lock() {
        Ok(object) => object,
        Err(err) => {
            eprintln!("ERROR: Failed to get value under text_status ARC!\n{err}");
            return;
        }
    };
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

    // drop the arc references since they are not needed anymore
    drop(input_user);
    drop(input_user_arc);
    drop(input_pass);
    drop(input_pass_arc);

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

///
/// Callback function called when you resize the window
/// Basically makes sure the ui is adaptive with the window's size.
///
pub fn resize_callback(
    w: i32,
    h: i32,
    title: &mut Frame,
    back_button_arc: Arc<Mutex<button::Button>>,
    input_user_arc: Arc<Mutex<input::Input>>,
    input_pass_arc: Arc<Mutex<input::Input>>,
    but_login_arc: Arc<Mutex<button::Button>>,
    text_status_arc: Arc<Mutex<text::TextDisplay>>,
    flex: &mut Flex,
) {
    let w_center = w / 2;
    let h_center = h / 2;
    let font_size = (f32::sqrt(w as f32 * h as f32) / 20.0).floor() as i32;

    title.set_label_size(font_size * 2);
    title.set_pos((w / 2) - (font_size / 24), font_size);

    match back_button_arc.lock() {
        Ok(mut o) => {
            o.set_label_size(font_size / 2);
        }
        Err(err) => {
            eprintln!(
                "ERROR: There was an error changing back_button text size, arc poison error!\n{err}"
            );
        }
    };

    match input_user_arc.lock() {
        Ok(mut o) => {
            o.set_text_size(font_size);
        }
        Err(err) => {
            eprintln!(
                "ERROR: There was an error changing input_user text size, arc poison error!\n{err}"
            );
        }
    };

    match input_pass_arc.lock() {
        Ok(mut o) => {
            o.set_text_size(font_size);
        }
        Err(err) => {
            eprintln!(
                "ERROR: There was an error changing input_pass text size, arc poison error!\n{err}"
            );
        }
    };

    match but_login_arc.lock() {
        Ok(mut o) => {
            o.set_label_size(font_size);
        }
        Err(err) => {
            eprintln!(
                "ERROR: There was an error changing but_login text size, arc poison error!\n{err}"
            );
        }
    };

    match text_status_arc.lock() {
        Ok(mut o) => {
            o.set_text_size(font_size / 3);
        }
        Err(err) => {
            eprintln!("ERROR: There was an error changing text_status text size, arc poison error!\n{err}");
        }
    };

    flex.resize(w_center / 2, h_center / 2, w_center, h_center);
}
