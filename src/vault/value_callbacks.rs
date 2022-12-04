use super::core::UpdateContentErr;
use super::core::{VaultValue, VaultValueErr};
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
    let install_path_value: String;
    if install_path.active() {
        if !utils::is_path_os_valid(&install_path.value()) {
            status_label.set_label("The given path is invalid on the current operating system!");
            status_label.show();
            return;
        } else {
            install_path_value = install_path.value().to_string();
        }
    } else {
        install_path_value = String::new();
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
    drop(install_path);
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

    if !install_path.active() {
        status_label.set_label("There is no install path active for this entry!");
        status_label.show();
        return;
    }

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
/// Callback function called when you hit select file
/// Basically it brings up the native file explorer,
/// it falls back to fltk's one if needed
/// It lets you select a file and if its content is not 
/// bigger then 32767 bytes and it's in utf8 it will display
/// the content in the content box, otherwise it will save
/// the content in the database, deactivate the content box
/// and display a placeholder message telling you this.
///
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
