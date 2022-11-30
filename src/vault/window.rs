use crate::utils::constants;
use crate::utils::Vault;
use crate::ECIES;
use fltk::{prelude::*, window::Window, *};
use sled;
use std::collections::HashMap;
use std::process;
use std::str;
use std::sync::{Arc, Mutex};

pub fn create(is_windows: bool, vault: Vault) -> fltk::window::DoubleWindow {
    // Create vault window
    let wind = Window::default().with_size(1000, 500).with_label("Vault");

    // entries coloumn
    let mut entries = tree::Tree::default().with_size(200, 475);
    let entries_arc = Arc::new(Mutex::new(entries.clone()));

    // add entrie portion
    let mut entrie_add_input = fltk::input::Input::default()
        .with_size(175, 25)
        .below_of(&entries, 0);
    entrie_add_input.set_color(entrie_add_input.color().lighter());
    let mut entrie_add_button = fltk::button::Button::default()
        .with_size(25, 25)
        .right_of(&entrie_add_input, 0)
        .with_label("+");

    let mut entrie_name = fltk::frame::Frame::default()
        .with_size(750, 25)
        .right_of(&entries, 25);
    entrie_name.set_label_size(30);
    entrie_name.set_pos(entrie_name.x(), 20);
    entrie_name.hide();

    // vault entry value portion
    let mut install_path_label = fltk::frame::Frame::default()
        .with_size(750, 20)
        .below_of(&entrie_name, 5);
    install_path_label.set_label_size(20);
    install_path_label.set_label("Install Path");
    install_path_label.hide();
    let mut install_path = fltk::input::Input::default()
        .with_size(750, 20)
        .below_of(&install_path_label, 1);
    install_path.set_color(install_path.color().lighter());
    install_path.set_text_size(15);
    install_path.hide();
    let install_path_arc = Arc::new(Mutex::new(install_path.clone()));

    let mut content_label = fltk::frame::Frame::default()
        .with_size(750, 20)
        .below_of(&install_path, 5);
    content_label.set_label_size(20);
    content_label.set_label("Content");
    content_label.hide();
    let mut content = fltk::input::MultilineInput::default()
        .with_size(750, 150)
        .below_of(&content_label, 1);
    content.set_color(content.color().lighter());
    content.set_text_size(15);
    content.hide();
    let content_arc = Arc::new(Mutex::new(content.clone()));

    let mut notes_label = fltk::frame::Frame::default()
        .with_size(750, 20)
        .below_of(&content, 5);
    notes_label.set_label_size(20);
    notes_label.set_label("Notes");
    notes_label.hide();
    let mut notes = fltk::input::MultilineInput::default()
        .with_size(750, 150)
        .below_of(&notes_label, 1);
    notes.set_color(notes.color().lighter());
    notes.set_text_size(15);
    notes.hide();
    let notes_arc = Arc::new(Mutex::new(notes.clone()));

    let mut save_button = fltk::button::Button::default()
        .with_size(75, 25)
        .below_of(&notes, 5);
    save_button.set_pos(900, save_button.y());
    save_button.set_label("Save");
    save_button.hide();
    let save_button_arc = Arc::new(Mutex::new(save_button.clone()));

    let mut delete_button = fltk::button::Button::default()
        .with_size(75, 25)
        .below_of(&notes, 5);
    delete_button.set_label("Delete");
    delete_button.hide();
    let delete_button_arc = Arc::new(Mutex::new(delete_button.clone()));

    let mut install_button = fltk::button::Button::default()
        .with_size(75, 25)
        .below_of(&notes, 5);
    install_button.set_label("Install");
    install_button.set_pos(562, install_button.y());
    install_button.hide();
    let install_button_arc = Arc::new(Mutex::new(install_button.clone()));

    let mut error_label = fltk::frame::Frame::default()
        .with_size(750, 20)
        .below_of(&notes, 5);
    error_label.set_label_size(14);
    error_label.set_pos(error_label.x(), error_label.y() + 25);
    error_label.hide();
    let error_label_arc = Arc::new(Mutex::new(error_label.clone()));

    // End customizing window
    wind.end();

    // create some needed objects
    let ecies = Arc::new(Mutex::new(ECIES::new()));

    // load vault
    let vaults_root_path;
    if is_windows {
        vaults_root_path = constants::VAULTS_ROOT_PATH_WINDOWS.to_string();
    } else {
        vaults_root_path = constants::VAULTS_ROOT_PATH_UNIX.to_string();
    }
    let vault_db = Arc::new(Mutex::new(
        match sled::open(vaults_root_path + &vault.id.to_string()) {
            Ok(db) => db,
            Err(err) => {
                eprintln!("ERROR: Even though the db appears in the list with db's there isn't one that's actually available in storage, or maybe there has been some one-time error, please try again!\n{err}");
                process::exit(100);
            }
        },
    ));

    // load current entries in db and display them
    // + save them in a dict where the key is the unecrypted value and the value is the one
    // encrypted

    // get value under vault_db arc
    let vault_db_locked = match vault_db.lock() {
        Ok(db) => db,
        Err(err) => {
            eprintln!("ERROR: For some reason we can't get the value for vault_db under ARC, even though we just created and it hasn't been used anywhere else, please try again!\n{err}");
            process::exit(100);
        }
    };

    let mut ecies_locked = match ecies.lock() {
        Ok(object) => object,
        Err(err) => {
            eprintln!("ERROR: Failed to get value under ecies ARC!\n{err}");
            process::exit(100);
        }
    };

    let db_entries = vault_db_locked.iter();
    let db_entries_dict = Arc::new(Mutex::new(HashMap::new()));

    let mut db_entries_dict_locked = match db_entries_dict.lock() {
        Ok(object) => object,
        Err(err) => {
            eprintln!("ERROR: Failed to get the value under db_entries_dict ARC!\n{err}");
            process::exit(100);
        }
    };

    for entry in db_entries {
        let current_entry_encrypted = match entry {
            Ok(cipher) => cipher.0.to_vec(),
            Err(err) => {
                eprintln!("ERROR: Failed to get an entry that we just read, this error message should not be displayed, but if for some reason is, just try again, or post an issue on github!\n{err}");
                process::exit(100);
            }
        };
        let current_entry_plain = match ecies_locked.decrypt_bytes(
            &current_entry_encrypted,
            &vault.priv_key,
            &vault.pub_key,
        ) {
            Ok(plain) => plain,
            Err(err) => {
                eprintln!("ERROR: Failed to decrypt the entry even though the vault keys were validated, try again, if it doesn't work then the db might be corrupted!\n{err}");
                process::exit(100);
            }
        };
        let current_entry_string = match str::from_utf8(&current_entry_plain) {
            Ok(s) => s,
            Err(err) => {
                eprintln!("ERROR: Failed to convert the decrypted bytes into a string, your current db is most likely corrupted, try again maybe!\n{err}");
                process::exit(100);
            }
        };
        entries.add(current_entry_string);

        db_entries_dict_locked.insert(current_entry_string.to_owned(), current_entry_encrypted);
    }

    drop(vault_db_locked);
    drop(ecies_locked);
    drop(db_entries_dict_locked);

    // global value for callbacks (for storing the currently selected entry)
    let current_selected_entry = Arc::new(Mutex::new(String::new()));

    // clone the arc references
    let vault_arc_clone = Arc::new(Mutex::new(vault.clone()));
    let db_entries_dict_arc_clone = db_entries_dict.clone();
    let current_selected_entry_arc_clone = current_selected_entry.clone();
    let ecies_arc_clone = ecies.clone();
    let notes_arc_clone = notes_arc.clone();
    let install_path_arc_clone = install_path_arc.clone();
    let content_arc_clone = content_arc.clone();
    let vault_db_arc_clone = vault_db.clone();
    let error_label_arc_clone = error_label_arc.clone();
    // Window callbacks
    // set entries callback
    entries.set_callback(move |e| {
        super::callbacks::entries_callback(
            e,
            &mut entrie_name,
            &mut install_path_label,
            install_path_arc_clone.clone(),
            &mut content_label,
            content_arc_clone.clone(),
            &mut notes_label,
            notes_arc_clone.clone(),
            save_button_arc.clone(),
            delete_button_arc.clone(),
            install_button_arc.clone(),
            error_label_arc_clone.clone(),
            current_selected_entry_arc_clone.clone(),
            db_entries_dict_arc_clone.clone(),
            vault_db_arc_clone.clone(),
            ecies_arc_clone.clone(),
            vault_arc_clone.clone(),
        );
    });

    // clone the arc references
    let vault_arc_clone = Arc::new(Mutex::new(vault.clone()));
    let ecies_arc_clone = ecies.clone();
    let db_entries_dict_arc_clone = db_entries_dict.clone();
    let vault_db_arc_clone = vault_db.clone();
    let entries_arc_clone = entries_arc.clone();
    // set entrie add button callback
    entrie_add_button.set_callback(move |_| {
        super::callbacks::entrie_add_button_callback(
            &mut entrie_add_input,
            vault_db_arc_clone.clone(),
            vault_arc_clone.clone(),
            ecies_arc_clone.clone(),
            db_entries_dict_arc_clone.clone(),
            entries_arc_clone.clone(),
        );
    });

    // clone the arc references
    let vault_arc_clone = Arc::new(Mutex::new(vault.clone()));
    let current_selected_entry_arc_clone = current_selected_entry.clone();
    let db_entries_dict_arc_clone = db_entries_dict.clone();
    let ecies_arc_clone = ecies.clone();
    let vault_db_arc_clone = vault_db.clone();
    let notes_arc_clone = notes_arc.clone();
    let install_path_arc_clone = install_path_arc.clone();
    let content_arc_clone = content_arc.clone();
    let error_label_arc_clone = error_label_arc.clone();
    // set save button callback
    save_button.set_callback(move |_| {
        super::callbacks::save_button_callback(
            install_path_arc_clone.clone(),
            content_arc_clone.clone(),
            notes_arc_clone.clone(),
            error_label_arc_clone.clone(),
            current_selected_entry_arc_clone.clone(),
            vault_db_arc_clone.clone(),
            vault_arc_clone.clone(),
            db_entries_dict_arc_clone.clone(),
            ecies_arc_clone.clone(),
        );
    });

    let current_selected_entry_arc_clone = current_selected_entry.clone();
    let db_entries_dict_arc_clone = db_entries_dict.clone();
    let entries_arc_clone = entries_arc.clone();
    let vault_db_arc_clone = vault_db.clone();
    // set delete button callback
    delete_button.set_callback(move |_| {
        super::callbacks::delete_button_callback(
            vault_db_arc_clone.clone(),
            current_selected_entry_arc_clone.clone(),
            db_entries_dict_arc_clone.clone(),
            entries_arc_clone.clone(),
        );
    });

    let install_path_arc_clone = install_path_arc.clone();
    let content_arc_clone = content_arc.clone();
    let error_label_arc_clone = error_label_arc.clone();
    // set install button callback
    install_button.set_callback(move |_| {
        super::callbacks::install_button_callback(
            install_path_arc_clone.clone(),
            content_arc_clone.clone(),
            error_label_arc_clone.clone(),
            is_windows,
        );
    });

    wind
}
