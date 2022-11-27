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
use fltk_theme::{color_themes, ColorTheme, SchemeType, WidgetScheme};
use crate::{utils::db, encrypter::{ecies::ECIES, key_encrypt::KeyEncrypt}, hasher::argon2id::Argon2id};
use crate::hasher::blake3;
use crate::hasher::argon2id;
use std::sync::{Arc, Mutex};
use crate::utils::Vault;
use fltk_grid::Grid;
use sled;
use crate::utils::constants;
use std::str;
use super::core::VaultValue;
use std::collections::HashMap;

pub fn create(is_windows: bool, vault: Vault) -> fltk::window::DoubleWindow {
    // Create vault window
    let mut wind = Window::default().with_size(1000, 500).with_label("Vault");
    // let mut flex = Flex::default()
    //     .with_size(950, 450)
    //     .center_of_parent()
    //     .column();
    
    // create some needed objects
    let mut ecies = ECIES::new();
    let mut ecies2 = ECIES::new();
    let mut ecies3 = ECIES::new();
    
    // open vault
    // TODO: handle errors
    let mut vaults_root_path = String::from("");
    if is_windows {
        vaults_root_path = constants::VAULTS_ROOT_PATH_WINDOWS.to_string();
    } else {
        vaults_root_path = constants::VAULTS_ROOT_PATH_UNIX.to_string();
    }

    let vault_db = sled::open(vaults_root_path + &vault.id.to_string()).unwrap();
    let vault_db_2 = vault_db.clone();
    let vault_db_3 = vault_db.clone();

    // entries coloumn
    let mut entries = tree::Tree::default().with_size(200, 475);

    // load current entries in db and display them
    // + save them in a dict where the key is the unecrypted value and the value is the one
    // encrypted
    let db_entries = vault_db.iter();
    let mut db_entries_dict = Arc::new(Mutex::new(HashMap::new()));
    for entry in db_entries {
        // TODO: handle errors
        let current_entry_encrypted = entry.unwrap().0.to_vec();
        let current_entry_plain = ecies.decrypt_bytes(&current_entry_encrypted, &vault.priv_key, &vault.pub_key).unwrap();
        let current_entry_string = str::from_utf8(&current_entry_plain).unwrap();
        entries.add(current_entry_string);
        db_entries_dict.lock().unwrap().insert(current_entry_string.to_owned(), current_entry_encrypted);
    }

    // add entrie portion
    let mut entrie_add_input = fltk::input::Input::default().with_size(175, 25).below_of(&entries, 0);
    entrie_add_input.set_color(entrie_add_input.color().lighter());
    let mut entrie_add_button = fltk::button::Button::default().with_size(25, 25).right_of(&entrie_add_input, 0).with_label("+");

    // create a grid for the content inside the entrie
    // let mut grid = Grid::default_fill();
    // grid.debug(true); // set to true to show cell outlines and numbers
    // grid.set_layout(5, 1); // 5 rows, 5 columns

    // values in entrie
    // let mut entrie_name = fltk::text::TextDisplay::default().with_size(700, 25).right_of(&entries, 50);
    // entrie_name.set_color(entrie_name.color().lighter());
    // entrie_name.set_text_size(15);
    // entrie_name.set_pos(entrie_name.x(), 20);

    let mut entrie_name = fltk::frame::Frame::default().with_size(750, 25).right_of(&entries, 25);
    entrie_name.set_label_size(30);
    entrie_name.set_pos(entrie_name.x(), 20);
    entrie_name.hide();

    let mut install_path_label = fltk::frame::Frame::default().with_size(750, 20).below_of(&entrie_name, 5);
    install_path_label.set_label_size(20);
    install_path_label.set_label("Install Path");
    install_path_label.hide();
    let mut install_path = fltk::input::Input::default().with_size(750, 20).below_of(&install_path_label, 1);
    install_path.set_color(install_path.color().lighter());
    install_path.set_text_size(15);
    install_path.hide();
    let install_path_arc = Arc::new(Mutex::new(install_path.clone()));

    let mut content_label = fltk::frame::Frame::default().with_size(750, 20).below_of(&install_path, 5);
    content_label.set_label_size(20);
    content_label.set_label("Content");
    content_label.hide();
    let mut content = fltk::input::MultilineInput::default().with_size(750, 150).below_of(&content_label, 1);
    content.set_color(content.color().lighter());
    content.set_text_size(15);
    content.hide();
    let content_arc = Arc::new(Mutex::new(content.clone()));

    let mut notes_label = fltk::frame::Frame::default().with_size(750, 20).below_of(&content, 5);
    notes_label.set_label_size(20);
    notes_label.set_label("Notes");
    notes_label.hide();
    let mut notes = fltk::input::MultilineInput::default().with_size(750, 150).below_of(&notes_label, 1);
    notes.set_color(notes.color().lighter());
    notes.set_text_size(15);
    notes.hide();
    let notes_arc = Arc::new(Mutex::new(notes.clone()));

    let mut save_button = fltk::button::Button::default().with_size(75, 25).below_of(&notes, 5);
    save_button.set_pos(900, save_button.y());
    save_button.set_label("Save");
    save_button.hide();
    let save_button_arc = Arc::new(Mutex::new(save_button.clone()));

    // End customizing window
    // flex.end();
    wind.end();

    // global value for callbacks
    let current_selected_entry = Arc::new(Mutex::new(String::new()));

    let db_entries_dict_arc_clone = db_entries_dict.clone();
    let current_selected_entry_arc_clone = current_selected_entry.clone();
    // Window callbacks
    entries.set_callback(move |e| {
        let selected_item = &e.first_selected_item().unwrap().label().unwrap();
        *current_selected_entry_arc_clone.lock().unwrap() = selected_item.to_owned();

        if selected_item == "ROOT" {
            entrie_name.hide();
            install_path_label.hide();
            install_path.hide();
            content_label.hide();
            content.hide();
            notes_label.hide();
            notes.hide();
            save_button_arc.lock().unwrap().hide();
            panic!(); // using panic to leave callback, because of fltk's nature this works with no
                      // problem
        }

        // TODO: handle errors
        let entry_value_encrypted = vault_db.get(db_entries_dict_arc_clone.lock().unwrap().get(selected_item).unwrap()).unwrap().unwrap().to_vec();
        let entry_value_decrypted = ecies2.decrypt_bytes(&entry_value_encrypted, &vault.priv_key, &vault.pub_key).unwrap();
        let entry_value_string = str::from_utf8(&entry_value_decrypted).unwrap();
        // parase entry value into json
        let entry_value_json: VaultValue = serde_json::from_str(entry_value_string).unwrap();

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
        save_button_arc.lock().unwrap().show();
        // TODO: handle error
        entrie_name.set_label(selected_item);
    });

    let db_entries_dict_arc_clone = db_entries_dict.clone();
    entrie_add_button.set_callback(move |b| {
        let entrie_add_input_value = &entrie_add_input.value();

        let mut vaults_root_path = String::from("");
        if is_windows {
            vaults_root_path = constants::VAULTS_ROOT_PATH_WINDOWS.to_string();
        } else {
            vaults_root_path = constants::VAULTS_ROOT_PATH_UNIX.to_string();
        }

        if entrie_add_input_value != "ROOT" {
            // TODO: handle errors
            let entrie_add_input_value_encrypted = ecies.encrypt_bytes_array(entrie_add_input_value.as_bytes(), &vault.priv_key, &vault.pub_key).unwrap();
            let empty_value = VaultValue::new_empty(); 
            let emtpy_value_string = serde_json::to_string(&empty_value).unwrap();
            let emtpy_value_encrypted = ecies.encrypt_bytes_array(emtpy_value_string.as_bytes(), &vault.priv_key, &vault.pub_key).unwrap();

            vault_db_2.insert((&entrie_add_input_value_encrypted).to_owned(), emtpy_value_encrypted).unwrap();

            db_entries_dict_arc_clone.lock().unwrap().insert(entrie_add_input_value.to_string(), entrie_add_input_value_encrypted);

            entries.add(entrie_add_input_value);
            entries.redraw();
        } else {
            entrie_add_input.set_value("name ROOT not allowed");
        }
    });

    let current_selected_entry_arc_clone = current_selected_entry.clone();
    let db_entries_dict_arc_clone = db_entries_dict.clone();
    save_button.set_callback(move |_| {
        let selected_item = current_selected_entry_arc_clone.lock().unwrap().to_owned();

        let install_path_value = install_path_arc.lock().unwrap().value();
        let content_value = content_arc.lock().unwrap().value();
        let notes_value = notes_arc.lock().unwrap().value();

        let entry_value = VaultValue {
            install_path: install_path_value,
            content: content_value,
            notes: notes_value,
        };

        let entry_value_string = serde_json::to_string(&entry_value).unwrap();

        let entry_value_encrypted = ecies3.encrypt_bytes_array(&entry_value_string.as_bytes(), &vault.priv_key, &vault.pub_key).unwrap();

        vault_db_3.insert(db_entries_dict_arc_clone.lock().unwrap().get(&selected_item).unwrap(), entry_value_encrypted).unwrap();
    });
    
    wind
}
