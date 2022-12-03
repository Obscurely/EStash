use crate::utils;
use fltk::{prelude::*, *};
use std::sync::{Arc, Mutex};

///
/// Callback function for when you hit the check entry button.
/// Checks if the path is a valid one for the current OS you are using.
///
pub fn install_path_check_button_callback(
    status_label_arc: Arc<Mutex<frame::Frame>>,
    install_path_arc_clone: Arc<Mutex<input::Input>>,
) {
    // get the actual object from arcs
    let mut status_label = match status_label_arc.lock() {
        Ok(object) => object,
        Err(err) => {
            eprintln!("ERROR: Failed to get value under error_label_arc ARC!\n{err}");
            return;
        }
    };
    let install_path = match install_path_arc_clone.lock() {
        Ok(object) => object,
        Err(err) => {
            eprintln!("ERROR: Failed to get value under install_path_arc ARC!\n{err}");
            status_label.set_label("There was a Poison Error, try again, or try to restart!");
            status_label.show();
            return;
        }
    };

    if !utils::is_path_os_valid(&install_path.value()) {
        status_label.set_label("The given path is invalid on the current operating system!");
        status_label.show();
        return;
    } else {
        status_label.set_label("The given path is valid on the current operating system!");
        status_label.show();
        return;
    }
}

///
/// Callback used when resizing the window.
/// Change buttons, texts size etc.
///
pub fn wind_resize_callback(
    w: i32,
    h: i32,
    entrie_add_input_arc: Arc<Mutex<input::Input>>,
    entrie_add_button_arc: Arc<Mutex<button::Button>>,
    entrie_name_arc: Arc<Mutex<frame::Frame>>,
    install_path_label_arc: Arc<Mutex<frame::Frame>>,
    install_path_arc: Arc<Mutex<input::Input>>,
    install_path_check_button_arc: Arc<Mutex<button::Button>>,
    content_label_arc: Arc<Mutex<frame::Frame>>,
    content_arc: Arc<Mutex<input::MultilineInput>>,
    delete_content_button_arc: Arc<Mutex<button::Button>>,
    select_file_button_arc: Arc<Mutex<button::Button>>,
    notes_label_arc: Arc<Mutex<frame::Frame>>,
    notes_arc: Arc<Mutex<input::MultilineInput>>,
    delete_button_arc: Arc<Mutex<button::Button>>,
    install_button_arc: Arc<Mutex<button::Button>>,
    save_button_arc: Arc<Mutex<button::Button>>,
    status_label_arc: Arc<Mutex<frame::Frame>>,
) {
    let font_size = (f32::sqrt(w as f32 * h as f32) / 20.0).floor() as i32;

    match entrie_add_input_arc.lock() {
        Ok(mut o) => {
            o.set_text_size(font_size / 3);
        }
        Err(err) => {
            eprintln!(
                "ERROR: There was an error getting value behind entrie_add_input ARC!\n {err}"
            );
        }
    };

    match entrie_add_button_arc.lock() {
        Ok(mut o) => {
            o.set_label_size(font_size / 3);
        }
        Err(err) => {
            eprintln!(
                "ERROR: There was an error getting value behind entrie_add_button ARC!\n {err}"
            );
        }
    };

    match entrie_name_arc.lock() {
        Ok(mut o) => {
            o.set_label_size(font_size);
        }
        Err(err) => {
            eprintln!(
                "ERROR: There was an error getting value behind entrie_name_arc ARC!\n {err}"
            );
        }
    };

    match install_path_label_arc.lock() {
        Ok(mut o) => {
            o.set_label_size(font_size / 2);
        }
        Err(err) => {
            eprintln!(
                "ERROR: There was an error getting value behind install_path_label ARC!\n {err}"
            );
        }
    };

    match install_path_arc.lock() {
        Ok(mut o) => {
            o.set_text_size(font_size / 3);
        }
        Err(err) => {
            eprintln!(
                "ERROR: There was an error getting value behind install_path_arc ARC!\n {err}"
            );
        }
    };

    match install_path_check_button_arc.lock() {
        Ok(mut o) => {
            o.set_label_size(font_size / 3);
        }
        Err(err) => {
            eprintln!("ERROR: There was an error getting value behind install_path_checK_button_arc ARC!\n {err}");
        }
    };

    match content_label_arc.lock() {
        Ok(mut o) => {
            o.set_label_size(font_size / 2);
        }
        Err(err) => {
            eprintln!(
                "ERROR: There was an error getting value behind content_label_arc ARC!\n {err}"
            );
        }
    };

    match content_arc.lock() {
        Ok(mut o) => {
            o.set_text_size(font_size / 3);
        }
        Err(err) => {
            eprintln!("ERROR: There was an error getting value behind content_arc ARC!\n {err}");
        }
    };

    match delete_content_button_arc.lock() {
        Ok(mut o) => {
            o.set_label_size(font_size / 3);
        }
        Err(err) => {
            eprintln!("ERROR: There was an error getting value behind delete_content_button_arc ARC!\n {err}");
        }
    };

    match select_file_button_arc.lock() {
        Ok(mut o) => {
            o.set_label_size(font_size / 3);
        }
        Err(err) => {
            eprintln!("ERROR: There was an error getting value behind select_file_button_arc ARC!\n {err}");
        }
    };

    match notes_label_arc.lock() {
        Ok(mut o) => {
            o.set_label_size(font_size / 2);
        }
        Err(err) => {
            eprintln!(
                "ERROR: There was an error getting value behind notes_label_arc ARC!\n {err}"
            );
        }
    };

    match notes_arc.lock() {
        Ok(mut o) => {
            o.set_text_size(font_size / 3);
        }
        Err(err) => {
            eprintln!("ERROR: There was an error getting value behind notes_arc ARC!\n {err}");
        }
    };

    match delete_button_arc.lock() {
        Ok(mut o) => {
            o.set_label_size(font_size / 3);
        }
        Err(err) => {
            eprintln!(
                "ERROR: There was an error getting value behind delete_button_arc ARC!\n {err}"
            );
        }
    };

    match install_button_arc.lock() {
        Ok(mut o) => o.set_label_size(font_size / 3),
        Err(err) => {
            eprintln!(
                "ERROR: There was an error getting value behind install_button_arc ARC!\n {err}"
            );
        }
    };

    match save_button_arc.lock() {
        Ok(mut o) => {
            o.set_label_size(font_size / 3);
        }
        Err(err) => {
            eprintln!(
                "ERROR: There was an error getting value behind save_button_arc ARC!\n {err}"
            );
        }
    };

    match status_label_arc.lock() {
        Ok(mut o) => {
            o.set_label_size(font_size / 3);
        }
        Err(err) => {
            eprintln!(
                "ERROR: There was an error getting value behind status_label_arc ARC!\n {err}"
            );
        }
    };
}

pub fn delete_button_callback(content_arc: Arc<Mutex<input::MultilineInput>>, status_label_arc: Arc<Mutex<frame::Frame>>) {
    // get the actual object from arcs
    let mut status_label = match status_label_arc.lock() {
        Ok(object) => object,
        Err(err) => {
            eprintln!("ERROR: Failed to get value under error_label_arc ARC!\n{err}");
            return;
        }
    };
    match content_arc.lock() {
        Ok(mut o) => {
            o.set_value("");
            o.activate();
        }
        Err(err) => {
            eprintln!("ERROR: Failed to get value under content_arc ARC!\n{err}");
            status_label.set_label("There was a Poison Error, try again, or try to restart!");
            status_label.show();
            return;
        }
    }
}
