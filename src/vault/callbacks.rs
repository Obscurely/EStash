use super::core::VaultValue;
use crate::hasher::argon2id;
use crate::hasher::blake3;
use crate::utils::constants;
use crate::utils::Vault;
use crate::utils;
use crate::{
    encrypter::{ecies::ECIES, key_encrypt::KeyEncrypt},
    hasher::argon2id::Argon2id,
    utils::db,
};
use fltk::{
    app,
    button::Button,
    enums::{Align, Font},
    frame::Frame,
    group::Flex,
    prelude::*,
    window::Window,
    *,
};
use fltk_grid::Grid;
use fltk_theme::{color_themes, ColorTheme, SchemeType, WidgetScheme};
use sled;
use sled::Db;
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

pub fn entries_callback(
    entries: &mut tree::Tree,
    entrie_name: &mut frame::Frame,
    install_path_label: &mut frame::Frame,
    install_path_arc: Arc<Mutex<input::Input>>,
    content_label: &mut frame::Frame,
    content_arc: Arc<Mutex<input::MultilineInput>>,
    notes_label: &mut frame::Frame,
    notes_arc: Arc<Mutex<input::MultilineInput>>,
    save_button_arc: Arc<Mutex<button::Button>>,
    delete_button_arc: Arc<Mutex<button::Button>>,
    error_label_arc: Arc<Mutex<frame::Frame>>,
    current_selected_entry_arc_clone: Arc<Mutex<String>>,
    db_entries_dict_arc_clone: Arc<Mutex<HashMap<String, Vec<u8>>>>,
    vault_db_arc_clone: Arc<Mutex<Db>>,
    ecies_arc_clone: Arc<Mutex<ECIES>>,
    vault_arc_clone: Arc<Mutex<Vault>>,
) {
    let selected_item = match entries.first_selected_item() {
        Some(first_selected_item) => {
            match first_selected_item.label() {
                Some(label) => label,
                None => "".to_string(),
            }
        },
        None => "".to_string(),
    };

    let selected_item = selected_item.as_str();

    // get the actual object from arcs
    let mut install_path = install_path_arc.lock().unwrap();
    let mut content = content_arc.lock().unwrap();
    let mut notes = notes_arc.lock().unwrap();
    let mut save_button = save_button_arc.lock().unwrap();
    let mut delete_button = delete_button_arc.lock().unwrap();
    let mut error_label = error_label_arc.lock().unwrap();

    // save the currently selected item
    *current_selected_entry_arc_clone.lock().unwrap() = selected_item.to_owned();

    // if the user clicks on ROOT hide all the vault widgets and exit what of the callback
    if selected_item == "ROOT" || selected_item == "" {
        entrie_name.hide();
        install_path_label.hide();
        install_path.hide();
        content_label.hide();
        content.hide();
        notes_label.hide();
        notes.hide();
        save_button.hide();
        delete_button.hide();
        error_label.hide();
    } else {
        let entry_value_json = super::core::get_entry_value_plain(
            vault_db_arc_clone.clone(),
            ecies_arc_clone.clone(),
            vault_arc_clone.clone(),
            selected_item,
            db_entries_dict_arc_clone.clone(),
        );

        // set value
        install_path.set_value(&entry_value_json.install_path);
        content.set_value(&entry_value_json.content);
        notes.set_value(&entry_value_json.notes);

        // Unhide widgets
        entrie_name.show();
        install_path_label.show();
        install_path.show();
        content_label.show();
        content.show();
        notes_label.show();
        notes.show();
        save_button.show();
        delete_button.show();
        error_label.hide();
        // TODO: handle error
        entrie_name.set_label(selected_item);
    }
}

pub fn entrie_add_button_callback(
    entrie_add_input: &mut input::Input,
    vault_db_arc_clone: Arc<Mutex<Db>>,
    vault_arc_clone: Arc<Mutex<Vault>>,
    ecies_arc_clone: Arc<Mutex<ECIES>>,
    db_entries_dict_arc_clone: Arc<Mutex<HashMap<String, Vec<u8>>>>,
    entries_arc_clone: Arc<Mutex<tree::Tree>>,
    is_windows: bool,
) {
    // get reference from arc
    let mut entries = entries_arc_clone.lock().unwrap();

    let entrie_add_input_value = &entrie_add_input.value();

    let mut vaults_root_path = String::from("");
    if is_windows {
        vaults_root_path = constants::VAULTS_ROOT_PATH_WINDOWS.to_string();
    } else {
        vaults_root_path = constants::VAULTS_ROOT_PATH_UNIX.to_string();
    }

    if entrie_add_input_value != "ROOT" {
        let entrie_add_input_value_encrypted = super::core::add_new_entry(
            vault_db_arc_clone.clone(),
            vault_arc_clone.clone(),
            entrie_add_input_value,
            ecies_arc_clone.clone(),
        );

        db_entries_dict_arc_clone.lock().unwrap().insert(
            entrie_add_input_value.to_string(),
            entrie_add_input_value_encrypted,
        );

        entries.add(entrie_add_input_value);
        entries.redraw();
    } else {
        entrie_add_input.set_value("name ROOT not allowed");
    }
}

pub fn save_button_callback(
    install_path_arc: Arc<Mutex<input::Input>>,
    content_arc: Arc<Mutex<input::MultilineInput>>,
    notes_arc: Arc<Mutex<input::MultilineInput>>,
    error_label_arc: Arc<Mutex<frame::Frame>>,
    current_selected_entry_arc_clone: Arc<Mutex<String>>,
    vault_db_arc_clone: Arc<Mutex<Db>>,
    vault_arc_clone: Arc<Mutex<Vault>>,
    db_entries_dict_arc_clone: Arc<Mutex<HashMap<String, Vec<u8>>>>,
    ecies_arc_clone: Arc<Mutex<ECIES>>,
) {
    // get references from arcs
    let vault = vault_arc_clone.lock().unwrap();
    let selected_item = current_selected_entry_arc_clone.lock().unwrap().to_owned();
    let install_path_value = install_path_arc.lock().unwrap().value();
    let content_value = content_arc.lock().unwrap().value();
    let notes_value = notes_arc.lock().unwrap().value();
    let mut error_label = error_label_arc.lock().unwrap(); 

    // empty error label
    error_label.set_label("");

    // check if the given path is valid
    if !utils::is_path_os_valid(&install_path_value) {
        error_label.set_label("The given path is invalid on the current operating system!");
        error_label.show();
        return;
    }

    let entry_value = VaultValue {
        install_path: install_path_value,
        content: content_value,
        notes: notes_value,
    };

    let entry_value_string = serde_json::to_string(&entry_value).unwrap();

    let entry_value_encrypted = ecies_arc_clone
        .lock()
        .unwrap()
        .encrypt_bytes_array(
            &entry_value_string.as_bytes(),
            &vault.priv_key,
            &vault.pub_key,
        )
        .unwrap();

    vault_db_arc_clone
        .lock()
        .unwrap()
        .insert(
            db_entries_dict_arc_clone
                .lock()
                .unwrap()
                .get(&selected_item)
                .unwrap(),
            entry_value_encrypted,
        )
        .unwrap();
}

pub fn delete_button_callback(vault_db_arc_clone: Arc<Mutex<Db>>, current_selected_entry_arc_clone: Arc<Mutex<String>>, db_entries_dict_arc_clone: Arc<Mutex<HashMap<String, Vec<u8>>>>, entries_arc_clone: Arc<Mutex<tree::Tree>>) {
    let mut entries = entries_arc_clone.lock().unwrap();
    let current_selected_entry = current_selected_entry_arc_clone.lock().unwrap();
    let mut db_entries_dict = db_entries_dict_arc_clone.lock().unwrap();

    vault_db_arc_clone.lock().unwrap().remove(db_entries_dict.get(current_selected_entry.as_str()).unwrap());

    let entries_items = entries.get_items().unwrap();

    for item in entries_items {
        if item.label().unwrap() == current_selected_entry.as_str() {
            // entries.deselect_all(&item, false).unwrap();
            db_entries_dict.remove(&item.label().unwrap());
            entries.remove(&item).unwrap();
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

    entries.redraw();

    entries.do_callback();

    // entries.select(entries_items.get(0).unwrap().label().unwrap().as_str(), true).unwrap();
}
