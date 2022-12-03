use super::core::UpdateContentErr;
use super::core::{NewEntryErr, VaultValue, VaultValueErr};
use crate::utils;
use crate::utils::Vault;
use crate::ECIES;
use fltk::{prelude::*, *};
use sled;
use sled::Db;
use std::collections::HashMap;
use std::str;
use std::sync::{Arc, Mutex};
use std::{fs, process};

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

///
/// Callback function for when you hit the save button on an entry
/// Basically takes all the input from the text boxes and updates the database.
///
pub fn save_button_callback(
    install_path_arc: Arc<Mutex<input::Input>>,
    content_arc: Arc<Mutex<input::MultilineInput>>,
    notes_arc: Arc<Mutex<input::MultilineInput>>,
    status_label_arc: Arc<Mutex<frame::Frame>>,
    current_selected_entry_arc_clone: Arc<Mutex<String>>,
    vault_db_arc_clone: Arc<Mutex<Db>>,
    vault_arc_clone: Arc<Mutex<Vault>>,
    db_entries_dict_arc_clone: Arc<Mutex<HashMap<String, Vec<u8>>>>,
    ecies_arc_clone: Arc<Mutex<ECIES>>,
) {
    // get references from arcs
    let mut status_label = match status_label_arc.lock() {
        Ok(object) => object,
        Err(err) => {
            eprintln!("ERROR: Failed to get value under error_label_arc ARC!\n{err}");
            return;
        }
    };
    let install_path_value = match install_path_arc.lock() {
        Ok(object) => object.value(),
        Err(err) => {
            eprintln!("ERROR: Failed to get value under install_path_arc ARC!\n{err}");
            status_label.set_label("There was a Poison Error, try again, or try to restart!");
            status_label.show();
            return;
        }
    };
    let content = match content_arc.lock() {
        Ok(object) => object,
        Err(err) => {
            eprintln!("ERROR: Failed to get value under content_arc ARC!\n{err}");
            status_label.set_label("There was a Poison Error, try again, or try to restart!");
            status_label.show();
            return;
        }
    };
    let notes_value = match notes_arc.lock() {
        Ok(object) => object.value(),
        Err(err) => {
            eprintln!("ERROR: Failed to get value under notes_arc ARC!\n{err}");
            status_label.set_label("There was a Poison Error, try again, or try to restart!");
            status_label.show();
            return;
        }
    };

    // empty error label
    status_label.set_label("");

    // check if the given path is valid
    if !utils::is_path_os_valid(&install_path_value) {
        status_label.set_label("The given path is invalid on the current operating system!");
        status_label.show();
        return;
    }

    // if the content widget isn't active it mean that it has either a file that is too big or a
    // file that isn't parsable to utf8 meaning we don't display it in the content box and keep it
    // in memory instead, so if that's the case we take the value from memory, if not we just take
    // them from the widgets
    let entry_value: VaultValue;
    if content.active() {
        entry_value = VaultValue {
            install_path: install_path_value,
            content: content.value().as_bytes().to_vec(),
            notes: notes_value,
        };
    } else {
        let selected_item = match current_selected_entry_arc_clone.lock() {
            Ok(object) => object,
            Err(err) => {
                eprintln!("ERROR: Failed to get value under notes_arc ARC!\n{err}");
                status_label.set_label("There was a Poison Error, try again, or try to restart!");
                status_label.show();
                return;
            }
        };

        let current_entry_value = match super::core::get_entry_value_plain(
            vault_db_arc_clone.clone(),
            ecies_arc_clone.clone(),
            vault_arc_clone.clone(),
            &selected_item,
            db_entries_dict_arc_clone.clone(),
        ) {
            Ok(val) => val,
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

        drop(selected_item);

        entry_value = VaultValue {
            install_path: install_path_value,
            content: current_entry_value.content,
            notes: notes_value,
        };
    }

    // drop arc ref
    drop(install_path_arc);
    drop(content);
    drop(content_arc);
    drop(notes_arc);

    // shouldn't error, hopefully
    let entry_value_string = match serde_json::to_string(&entry_value) {
        Ok(s) => s,
        Err(err) => {
            eprintln!("ERROR: Somehow converting the struct to json error'd out, shouldn't have, here is the error,\n{err}");
            status_label.set_label(
                "There was an error converting your input to json, restart or try again!",
            );
            status_label.show();
            return;
        }
    };

    // get value under ecies_arc arc
    let mut ecies = match ecies_arc_clone.lock() {
        Ok(object) => object,
        Err(err) => {
            eprintln!("ERROR: Failed to get value under ecies_arc ARC!\n{err}");
            status_label.set_label("There was a Poison Error, try again, or try to restart!");
            status_label.show();
            return;
        }
    };

    // get value under arc
    let vault = match vault_arc_clone.lock() {
        Ok(object) => object,
        Err(err) => {
            eprintln!("ERROR: Failed to get value under vault_arc_clone ARC!\n{err}");
            status_label.set_label("There was a Poison Error, try again, or try to restart!");
            status_label.show();
            return;
        }
    };

    // there should be no way for this to error out since once the vault is loaded it means the
    // keys work
    let entry_value_encrypted = match ecies.encrypt_bytes_array(
        &entry_value_string.as_bytes(),
        &vault.priv_key,
        &vault.pub_key,
    ) {
        Ok(cipher) => cipher,
        Err(err) => {
            eprintln!("ERROR: Failed to encrypt the data input by user, there should be no way for this to error out since once the vault is loaded it means the keys work, anyways here is the error,\n{err}");
            status_label.set_label(
                "There was an error encrypting the data you input, try again or restart!",
            );
            status_label.show();
            return;
        }
    };

    // drop arc ref
    drop(vault);
    drop(vault_arc_clone);
    drop(ecies);
    drop(ecies_arc_clone);

    // get value under arc
    let db_entries_dict = match db_entries_dict_arc_clone.lock() {
        Ok(object) => object,
        Err(err) => {
            eprintln!("ERROR: Failed to get value under db_entries_dict_arc ARC!\n{err}");
            status_label.set_label("There was a Poison Error, try again, or try to restart!");
            status_label.show();
            return;
        }
    };

    // get value under arc
    let selected_item = match current_selected_entry_arc_clone.lock() {
        Ok(object) => object.to_owned(),
        Err(err) => {
            eprintln!("ERROR: Failed to get value under current_selected_entry_arc ARC!\n{err}");
            status_label.set_label("There was a Poison Error, try again, or try to restart!");
            status_label.show();
            return;
        }
    };

    let selected_item_encrypted = match db_entries_dict.get(&selected_item) {
        Some(cipher) => cipher.to_owned(),
        None => {
            eprintln!("ERROR: The Values In Memory are not in sync with the ones on screen!");
            status_label.set_label(
                "What's in memory is not in sync with what's in storage, please restart!",
            );
            status_label.show();
            return;
        }
    };

    // drop arc
    drop(selected_item);
    drop(current_selected_entry_arc_clone);
    drop(db_entries_dict);
    drop(db_entries_dict_arc_clone);

    // get value under arc
    let vault_db = match vault_db_arc_clone.lock() {
        Ok(object) => object,
        Err(err) => {
            eprintln!("ERROR: Failedt to get value under vault_db_arc ARC!\n{err}");
            status_label.set_label("There was a Poison Error, try again, or try to restart!");
            status_label.show();
            return;
        }
    };

    // drop arc ref
    drop(status_label);
    drop(status_label_arc);

    match vault_db.insert(selected_item_encrypted, entry_value_encrypted) {
        Ok(_) => (),
        Err(err) => {
            eprintln!("ERROR: There was an error storing the input value in the db, for some reason the db in not accesible, please report this error on github if you are able to replicate it!\n{err}");
            process::exit(100);
        }
    };
}

///
/// Callback function for when you hit the delete button on an entry
/// Deletes that entry from the database.
///
pub fn delete_button_callback(
    vault_db_arc_clone: Arc<Mutex<Db>>,
    current_selected_entry_arc_clone: Arc<Mutex<String>>,
    db_entries_dict_arc_clone: Arc<Mutex<HashMap<String, Vec<u8>>>>,
    entries_arc_clone: Arc<Mutex<tree::Tree>>,
) {
    let mut entries = match entries_arc_clone.lock() {
        Ok(object) => object,
        Err(err) => {
            eprintln!("ERROR: Failed to get value under entries_arc ARC!\n{err}");
            return;
        }
    };
    let current_selected_entry = match current_selected_entry_arc_clone.lock() {
        Ok(object) => object,
        Err(err) => {
            eprintln!("ERROR: Failed to get value under current_selected_entry_arc ARC!\n{err}");
            return;
        }
    };
    let mut db_entries_dict = match db_entries_dict_arc_clone.lock() {
        Ok(object) => object,
        Err(err) => {
            eprintln!("ERROR: Failed to get value under db_entries_dict_arc ARC!\n{err}");
            return;
        }
    };
    let vault_db = match vault_db_arc_clone.lock() {
        Ok(object) => object,
        Err(err) => {
            eprintln!("ERROR: Failed to get value under vault_db_arc ARC!\n{err}");
            return;
        }
    };

    let current_selected_entry_encrypted = match db_entries_dict
        .get(current_selected_entry.as_str())
    {
        Some(cipher) => cipher,
        None => {
            eprintln!("ERROR: What's on screen is not in sync with what's in memory for some reason, either try again or restart, if this error persists please report on github");
            return;
        }
    };

    match vault_db.remove(current_selected_entry_encrypted) {
        Ok(_) => (),
        Err(err) => {
            eprintln!("ERROR: What's in memory is not in sync with what's in storage, either try again or restart, if this error persisits please report on github!\n{err}");
            return;
        }
    };

    // drop arc ref
    drop(vault_db);
    drop(vault_db_arc_clone);

    let entries_items = match entries.get_items() {
        Some(items) => items,
        None => {
            eprintln!("ERROR: Failed to entries items, there may just be no more items...");
            return;
        }
    };

    for item in entries_items {
        let current_item_label = match item.label() {
            Some(label) => label,
            None => {
                eprintln!("ERROR: There was an error getting current entrie's label, moving on");
                continue;
            }
        };

        if current_item_label == current_selected_entry.as_str() {
            db_entries_dict.remove(&current_item_label);
            match entries.remove(&item) {
                Ok(_) => (),
                Err(err) => {
                    eprintln!("ERROR: There was an error removing an element from the entries tree, moving on\n{err}");
                }
            };
            entries.redraw();
            break;
        }
    }

    // drop the arc so it can be used by the callback functiont
    drop(current_selected_entry);
    drop(current_selected_entry_arc_clone);

    entries.clear();

    for entrie in db_entries_dict.keys() {
        entries.add(entrie);
    }

    // drop arc ref
    drop(db_entries_dict);

    entries.redraw();

    entries.do_callback();
}

///
/// Callback function for when you hit the install button on an entry.
/// Takes the install path from the current input (not what's saved)
/// and the content and writes it to that file, creating any needed folders.
///
pub fn install_button_callback(
    install_path_arc: Arc<Mutex<input::Input>>,
    content_arc: Arc<Mutex<input::MultilineInput>>,
    vault_arc_clone: Arc<Mutex<Vault>>,
    vault_db_arc_clone: Arc<Mutex<Db>>,
    ecies_arc_clone: Arc<Mutex<ECIES>>,
    current_selected_entry_arc_clone: Arc<Mutex<String>>,
    db_entries_dict_arc_clone: Arc<Mutex<HashMap<String, Vec<u8>>>>,
    status_label_arc: Arc<Mutex<frame::Frame>>,
    is_windows: bool,
) {
    // get references behind arc
    let mut status_label = match status_label_arc.lock() {
        Ok(object) => object,
        Err(err) => {
            eprintln!("ERROR: Failed to get value under error_label_arc ARC!\n{err}");
            return;
        }
    };
    let install_path = match install_path_arc.lock() {
        Ok(object) => object,
        Err(err) => {
            eprintln!("ERROR: Failed to get value under install_path_arc ARC!\n{err}");
            status_label.set_label("There was a Poison Error, try again, or try to restart!");
            status_label.show();
            return;
        }
    };
    let content = match content_arc.lock() {
        Ok(object) => object,
        Err(err) => {
            eprintln!("ERROR: Failed to get value under content_arc ARC!\n{err}");
            status_label.set_label("There was a Poison Error, try again, or try to restart!");
            status_label.show();
            return;
        }
    };

    let install_path_value = install_path.value().to_owned();
    let content_value: Vec<u8>;

    // if the content widget isn't active it mean that it has either a file that is too big or a
    // file that isn't parsable to utf8 meaning we don't display it in the content box and keep it
    // in memory instead, so if that's the case we take the value from memory, if not we just take
    // them from the widgets
    if content.active() {
        content_value = content.value().to_owned().as_bytes().to_vec();
    } else {
        let selected_item = match current_selected_entry_arc_clone.lock() {
            Ok(object) => object,
            Err(err) => {
                eprintln!("ERROR: Failed to get value under notes_arc ARC!\n{err}");
                status_label.set_label("There was a Poison Error, try again, or try to restart!");
                status_label.show();
                return;
            }
        };

        let current_entry_value = match super::core::get_entry_value_plain(
            vault_db_arc_clone.clone(),
            ecies_arc_clone.clone(),
            vault_arc_clone.clone(),
            &selected_item,
            db_entries_dict_arc_clone.clone(),
        ) {
            Ok(val) => val,
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

        drop(selected_item);

        content_value = current_entry_value.content;
    }

    // drop arc ref
    drop(install_path);
    drop(install_path_arc);
    drop(content);
    drop(content_arc);

    // check if install path is valid
    if !utils::is_path_os_valid(&install_path_value) {
        status_label.set_label("The given path is invalid on the current operating system!");
        status_label.show();
        return;
    }

    // create the folder or make sure there is one
    if is_windows {
        let mut path_folder_vec: Vec<&str> = install_path_value.split("\\").collect();
        path_folder_vec.pop();
        let path_folder = path_folder_vec.join("\\");

        match fs::create_dir_all(path_folder) {
            Ok(_) => (),
            Err(_) => {
                status_label
                    .set_label("There was an error creating/finding the dir where to install!");
                status_label.show();
                return;
            }
        };
    } else {
        let mut path_folder_vec: Vec<&str> = install_path_value.split("/").collect();
        path_folder_vec.pop();
        let path_folder = path_folder_vec.join("/");

        match fs::create_dir_all(path_folder) {
            Ok(_) => (),
            Err(_) => {
                status_label
                    .set_label("There was an error creating/finding the dir where to install!");
                status_label.show();
                return;
            }
        };
    }

    // try and write to that file
    match fs::write(install_path_value, content_value) {
        Ok(_) => {
            status_label.set_label("Successfully written the content to the file!");
            status_label.show();
            return;
        }
        Err(_) => {
            status_label.set_label("There was an error writing the content to the file!");
            status_label.show();
            return;
        }
    }
}

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

pub fn select_file_button(
    status_label_arc: Arc<Mutex<frame::Frame>>,
    content_arc: Arc<Mutex<input::MultilineInput>>,
    current_selected_entry_arc: Arc<Mutex<String>>,
    vault_db_arc: Arc<Mutex<Db>>,
    ecies_arc: Arc<Mutex<ECIES>>,
    vault_arc: Arc<Mutex<Vault>>,
    db_entries_dict_arc: Arc<Mutex<HashMap<String, Vec<u8>>>>,
) {
    let mut file_dialog = dialog::NativeFileChooser::new(dialog::NativeFileChooserType::BrowseFile);
    file_dialog.show();

    // get references from arcs
    let mut status_label = match status_label_arc.lock() {
        Ok(object) => object,
        Err(err) => {
            eprintln!("ERROR: Failed to get value under error_label_arc ARC!\n{err}");
            return;
        }
    };

    // get the content inside the file
    let file_content = match fs::read(file_dialog.filename()) {
        Ok(string) => string,
        Err(err) => {
            eprintln!(
                "ERROR: There was an error reading the contents of the selected file!\n{err}"
            );
            status_label.set_label("There was an error reading the contents of the file!");
            status_label.show();
            return;
        }
    };

    // get references from arcs
    let mut content = match content_arc.lock() {
        Ok(object) => object,
        Err(err) => {
            eprintln!("ERROR: Failed to get value under content_arc ARC!\n{err}");
            status_label.set_label("There was a Poison Error, try again, or try to restart!");
            status_label.show();
            return;
        }
    };

    // if the content is bigger then 32767 bytes then we save it to the database directly if
    // and deactivate the content box leaving a message if not we just set the content to the
    // box.
    if file_content.len() >= 32767 {
        content.set_value("File content is too big to be displayed!\nContent has been saved to the database automatically!");
        content.deactivate();

        match super::core::update_content_in_entry(
            current_selected_entry_arc.clone(),
            vault_db_arc.clone(),
            ecies_arc.clone(),
            vault_arc.clone(),
            db_entries_dict_arc.clone(),
            file_content,
        ) {
            Ok(_) => return,
            Err(UpdateContentErr::PoisonErr(_)) => {
                status_label.set_label("There was a Poison Error, try again, or try to restart!");
                status_label.show();
                return;
            }
            Err(UpdateContentErr::DisplayNotInSync(_)) => {
                status_label.set_label(
                    "What's on screen is not in sync with what's in memory, try again or restart!",
                );
                status_label.show();
                return;
            }
            Err(UpdateContentErr::MemoryNotInSync(_)) => {
                status_label.set_label(
                    "What's in memory is not in sync with what's in storage, please restart!",
                );
                status_label.show();
                return;
            }
            Err(UpdateContentErr::UnknownError(_)) => {
                process::exit(100);
            }
        };
    } else {
        match str::from_utf8(&file_content) {
            Ok(string) => {
                content.set_value(string);
                content.activate();
            }
            Err(_) => {
                content.set_value("Content is not in utf8 so it can't be displayed!\nThe content has been saved to database automatically!");
                content.deactivate();

                match super::core::update_content_in_entry(
                    current_selected_entry_arc.clone(),
                    vault_db_arc.clone(),
                    ecies_arc.clone(),
                    vault_arc.clone(),
                    db_entries_dict_arc.clone(),
                    file_content,
                ) {
                    Ok(_) => return,
                    Err(UpdateContentErr::PoisonErr(_)) => {
                        status_label
                            .set_label("There was a Poison Error, try again, or try to restart!");
                        status_label.show();
                        return;
                    }
                    Err(UpdateContentErr::DisplayNotInSync(_)) => {
                        status_label.set_label(
                            "What's on screen is not in sync with what's in memory, try again or restart!",
                        );
                        status_label.show();
                        return;
                    }
                    Err(UpdateContentErr::MemoryNotInSync(_)) => {
                        status_label.set_label(
                            "What's in memory is not in sync with what's in storage, please restart!",
                        );
                        status_label.show();
                        return;
                    }
                    Err(UpdateContentErr::UnknownError(_)) => {
                        process::exit(100);
                    }
                };
            }
        }
    }
}
