use super::core::{NewEntryErr, VaultValueErr};
use crate::utils::Vault;
use crate::ECIES;
use fltk::{prelude::*, *};
use sled;
use sled::Db;
use std::collections::HashMap;
use std::str;
use std::sync::{Arc, Mutex};
use std::process;

///
/// Callback for when you click on a different entry in the vault
///
pub fn entries_callback(
    entries: &mut tree::Tree,
    entrie_name_arc: Arc<Mutex<frame::Frame>>,
    install_path_label_arc: Arc<Mutex<frame::Frame>>,
    install_path_arc: Arc<Mutex<input::Input>>,
    install_path_check_button_arc: Arc<Mutex<button::Button>>,
    content_label_arc: Arc<Mutex<frame::Frame>>,
    content_arc: Arc<Mutex<input::MultilineInput>>,
    clear_content_button_arc: Arc<Mutex<button::Button>>,
    select_file_button_arc: Arc<Mutex<button::Button>>,
    notes_label_arc: Arc<Mutex<frame::Frame>>,
    notes_arc: Arc<Mutex<input::MultilineInput>>,
    save_button_arc: Arc<Mutex<button::Button>>,
    delete_button_arc: Arc<Mutex<button::Button>>,
    install_button_arc: Arc<Mutex<button::Button>>,
    status_label_arc: Arc<Mutex<frame::Frame>>,
    current_selected_entry_arc_clone: Arc<Mutex<String>>,
    db_entries_dict_arc_clone: Arc<Mutex<HashMap<String, Vec<u8>>>>,
    vault_db_arc_clone: Arc<Mutex<Db>>,
    ecies_arc_clone: Arc<Mutex<ECIES>>,
    vault_arc_clone: Arc<Mutex<Vault>>,
) {
    let selected_item = match entries.first_selected_item() {
        Some(first_selected_item) => match first_selected_item.label() {
            Some(label) => label,
            None => "".to_string(),
        },
        None => "".to_string(),
    };

    let selected_item = selected_item.as_str();

    // get the actual object from arcs
    let mut status_label = match status_label_arc.lock() {
        Ok(object) => object,
        Err(err) => {
            eprintln!("ERROR: Failed to get value under error_label_arc ARC!\n{err}");
            return;
        }
    };
    let mut entrie_name = match entrie_name_arc.lock() {
        Ok(object) => object,
        Err(err) => {
            eprintln!("ERROR: Failed to get value under entrie_name_arc ARC!\n{err}");
            status_label.set_label("There was a Poison Error, try again, or try to restart!");
            status_label.show();
            return;
        }
    };
    let mut install_path_label = match install_path_label_arc.lock() {
        Ok(object) => object,
        Err(err) => {
            eprintln!("ERROR: Failed to get value under install_path_label_arc ARC!\n{err}");
            status_label.set_label("There was a Poison Error, try again, or try to restart!");
            status_label.show();
            return;
        }
    };
    let mut install_path = match install_path_arc.lock() {
        Ok(object) => object,
        Err(err) => {
            eprintln!("ERROR: Failed to get value under install_path_arc ARC!\n{err}");
            status_label.set_label("There was a Poison Error, try again, or try to restart!");
            status_label.show();
            return;
        }
    };
    let mut install_path_check_button = match install_path_check_button_arc.lock() {
        Ok(object) => object,
        Err(err) => {
            eprintln!("ERROR: Failed to get value under install_path_check_button_arc ARC!\n{err}");
            status_label.set_label("There was a Poison Error, try again, or try to restart!");
            status_label.show();
            return;
        }
    };
    let mut content_label = match content_label_arc.lock() {
        Ok(object) => object,
        Err(err) => {
            eprintln!("ERROR: Failed to get value under content_label_arc ARC!\n{err}");
            status_label.set_label("There was a Poison Error, try again, or try to restart!");
            status_label.show();
            return;
        }
    };
    let mut content = match content_arc.lock() {
        Ok(object) => object,
        Err(err) => {
            eprintln!("ERROR: Failed to get value under content_arc ARC!\n{err}");
            status_label.set_label("There was a Poison Error, try again, or try to restart!");
            status_label.show();
            return;
        }
    };
    let mut clear_content_button = match clear_content_button_arc.lock() {
        Ok(object) => object,
        Err(err) => {
            eprintln!("ERROR: Failed to get value under clear_content_button ARC!\n{err}");
            status_label.set_label("There was a Poison Error, try again, or try to restart!");
            status_label.show();
            return;
        }
    };
    let mut select_file_button = match select_file_button_arc.lock() {
        Ok(object) => object,
        Err(err) => {
            eprintln!("ERROR: Failed to get value under select_file_button_arc ARC!\n{err}");
            status_label.set_label("There was a Poison Error, try again, or try to restart!");
            status_label.show();
            return;
        }
    };
    let mut notes_label = match notes_label_arc.lock() {
        Ok(object) => object,
        Err(err) => {
            eprintln!("ERROR: Failed to get value under notes_label_arc ARC!\n{err}");
            status_label.set_label("There was a Poison Error, try again, or try to restart!");
            status_label.show();
            return;
        }
    };
    let mut notes = match notes_arc.lock() {
        Ok(object) => object,
        Err(err) => {
            eprintln!("ERROR: Failed to get value under notes_arc ARC!\n{err}");
            status_label.set_label("There was a Poison Error, try again, or try to restart!");
            status_label.show();
            return;
        }
    };
    let mut save_button = match save_button_arc.lock() {
        Ok(object) => object,
        Err(err) => {
            eprintln!("ERROR: Failed to get value under save_button_arc ARC!\n{err}");
            status_label.set_label("There was a Poison Error, try again, or try to restart!");
            status_label.show();
            return;
        }
    };
    let mut delete_button = match delete_button_arc.lock() {
        Ok(object) => object,
        Err(err) => {
            eprintln!("ERROR: Failed to get value under delete_button_arc ARC!\n{err}");
            status_label.set_label("There was a Poison Error, try again, or try to restart!");
            status_label.show();
            return;
        }
    };
    let mut install_button = match install_button_arc.lock() {
        Ok(object) => object,
        Err(err) => {
            eprintln!("ERROR: Failed to get value under install_button_arc ARC!\n{err}");
            status_label.set_label("There was a Poison Error, try again, or try to restart!");
            status_label.show();
            return;
        }
    };
    let mut current_selected_entry = match current_selected_entry_arc_clone.lock() {
        Ok(object) => object,
        Err(err) => {
            eprintln!("ERROR: Failed to get value under current_selected_entry_arc ARC!\n{err}");
            status_label.set_label("There was a Poison Error, try again, or try to restart!");
            status_label.show();
            return;
        }
    };

    // save the currently selected item
    *current_selected_entry = selected_item.to_owned();

    // drop arc ref to current selected entry
    drop(current_selected_entry);
    drop(current_selected_entry_arc_clone);

    // if the user clicks on ROOT hide all the vault widgets and exit what of the callback
    if selected_item == "ROOT" || selected_item == "" {
        entrie_name.hide();
        install_path_label.hide();
        install_path.hide();
        install_path_check_button.hide();
        content_label.hide();
        content.hide();
        clear_content_button.hide();
        select_file_button.hide();
        select_file_button.activate();
        notes_label.hide();
        notes.hide();
        save_button.hide();
        delete_button.hide();
        install_button.hide();
        status_label.hide();
    } else {
        let entry_value_json = match super::core::get_entry_value_plain(
            vault_db_arc_clone.clone(),
            ecies_arc_clone.clone(),
            vault_arc_clone.clone(),
            selected_item,
            db_entries_dict_arc_clone.clone(),
        ) {
            Ok(json) => json,
            Err(VaultValueErr::DbCorrupted(_)) => {
                process::exit(100);
            }
            Err(VaultValueErr::PoisonErr(_)) => {
                status_label.set_label("There was a Poison Error, try again, or try to restart!");
                status_label.show();
                return;
            }
            Err(VaultValueErr::DisplayNotInSync(_)) => {
                status_label.set_label(
                    "What's on screen is not in sync with what's in memory, try again or restart!",
                );
                status_label.show();
                return;
            }
            Err(VaultValueErr::MemoryNotInSync(_)) => {
                status_label.set_label(
                    "What's in memory is not in sync with what's in storage, please restart!",
                );
                status_label.show();
                return;
            }
        };

        // drop arc ref
        drop(vault_db_arc_clone);
        drop(ecies_arc_clone);
        drop(vault_arc_clone);
        drop(db_entries_dict_arc_clone);

        // set value
        install_path.set_value(&entry_value_json.install_path);

        if &entry_value_json.content.len() >= &32767 {
            content.set_value("File content is too big to be displayed!\nThe actual value is kept in the database.");
            content.deactivate();
        } else {
            match str::from_utf8(&entry_value_json.content) {
                Ok(string) => {
                    content.set_value(string);
                    content.activate();
                }
                Err(_) => {
                    content.set_value("Content is not in utf8 so it can't be displayed!\nThe actual value is kept in the database.");
                    content.deactivate();
                }
            }
        }

        notes.set_value(&entry_value_json.notes);

        // Unhide widgets
        entrie_name.show();
        install_path_label.show();
        install_path.show();
        install_path_check_button.show();
        content_label.show();
        content.show();
        clear_content_button.show();
        select_file_button.show();
        notes_label.show();
        notes.show();
        save_button.show();
        delete_button.show();
        install_button.show();
        status_label.hide();
        entrie_name.set_label(selected_item);
    }
}

///
/// Callback function for when you press the + button under the entries tree.
/// It takes the input from the box besides it and creates a new empty entry in the vault.
///
pub fn entrie_add_button_callback(
    entrie_add_input_arc: Arc<Mutex<input::Input>>,
    vault_db_arc_clone: Arc<Mutex<Db>>,
    vault_arc_clone: Arc<Mutex<Vault>>,
    ecies_arc_clone: Arc<Mutex<ECIES>>,
    db_entries_dict_arc_clone: Arc<Mutex<HashMap<String, Vec<u8>>>>,
    entries_arc_clone: Arc<Mutex<tree::Tree>>,
) {
    // get reference from arc
    let mut entries = match entries_arc_clone.lock() {
        Ok(object) => object,
        Err(err) => {
            eprintln!("ERROR: Failed to get value under entries_arc ARC!\n{err}");
            return;
        }
    };
    let mut entrie_add_input = match entrie_add_input_arc.lock() {
        Ok(object) => object,
        Err(err) => {
            eprintln!("ERROR: Failed to get value under entrie_add_input_arc ARC!\n{err}");
            return;
        }
    };

    let entrie_add_input_value = &entrie_add_input.value();

    match entries.find_item(&entrie_add_input_value) {
        Some(_) => return,
        None => (),
    };

    if entrie_add_input_value != "ROOT" {
        let entrie_add_input_value_encrypted = match super::core::add_new_entry(
            vault_db_arc_clone.clone(),
            vault_arc_clone.clone(),
            entrie_add_input_value,
            ecies_arc_clone.clone(),
        ) {
            Ok(cipher) => cipher,
            Err(NewEntryErr::DbCorrupted(_)) => {
                process::exit(100);
            }
            Err(NewEntryErr::DbInaccesible(_)) => {
                process::exit(100);
            }
            Err(NewEntryErr::UnknownError(_)) => {
                process::exit(100);
            }
            Err(NewEntryErr::PoisonErr(_)) => {
                return;
            }
        };

        // drop arc ref
        drop(vault_db_arc_clone);
        drop(vault_arc_clone);
        drop(ecies_arc_clone);

        match db_entries_dict_arc_clone.lock() {
            Ok(mut object) => {
                object.insert(
                    entrie_add_input_value.to_string(),
                    entrie_add_input_value_encrypted,
                );
            }
            Err(err) => {
                eprintln!("ERROR: Failed to get value under db_entries_dict_arc ARC!\n{err}");
                return;
            }
        }

        // drop arc ref
        drop(db_entries_dict_arc_clone);

        entries.add(entrie_add_input_value);
        entries.redraw();
    } else {
        entrie_add_input.set_value("name ROOT not allowed");
    }
}
