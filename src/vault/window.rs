use crate::utils::Vault;
use crate::ECIES;
use fltk::{
    prelude::*,
    window::{DoubleWindow, Window},
    *, enums::Color,
};
use std::sync::{Arc, Mutex};

pub fn create(
    is_windows: bool,
    vault: Vault,
    login_wind: &mut DoubleWindow,
) -> fltk::window::DoubleWindow {
    //
    //  Make window | UI Part
    //

    // Create vault window
    let mut wind = Window::default().with_size(1000, 500).with_label("Vault");
    wind.set_xclass("estash");

    // entries coloumn
    let mut entries = tree::Tree::default().with_size(200, 475);
    entries.set_color(Color::from_rgb(14, 14, 14));
    entries.set_selection_color(Color::from_rgb(140, 140, 140).darker());
    entries.set_item_label_fgcolor(Color::from_rgb(140, 140, 140));
    entries.set_show_root(false);
    entries.set_select_frame(enums::FrameType::FlatBox);
    entries.set_connector_color(Color::from_rgb(140, 140, 140));
    let entries_arc = Arc::new(Mutex::new(entries.clone()));

    // add entrie
    let mut entrie_add_input = fltk::input::Input::default()
        .with_size(175, 25)
        .below_of(&entries, 0);
    entrie_add_input.set_color(Color::from_rgb(31, 31, 31));
    entrie_add_input.set_text_color(Color::from_rgb(140, 140, 140));
    let entrie_add_input_arc = Arc::new(Mutex::new(entrie_add_input.clone()));
    let mut entrie_add_button = fltk::button::Button::default()
        .with_size(25, 25)
        .right_of(&entrie_add_input, 0)
        .with_label("+");
    entrie_add_button.set_color(Color::from_rgb(43, 43, 43));
    entrie_add_button.set_label_color(Color::from_rgb(140, 140, 140));
    let entrie_add_button_arc = Arc::new(Mutex::new(entrie_add_button.clone()));

    // entry name
    let mut entrie_name = fltk::frame::Frame::default()
        .with_size(750, 35)
        .right_of(&entries, 25);
    entrie_name.set_label_size(30);
    entrie_name.set_pos(entrie_name.x(), 10);
    entrie_name.hide();
    entrie_name.set_label_color(Color::from_rgb(140, 140, 140));
    let entrie_name_arc = Arc::new(Mutex::new(entrie_name.clone()));

    // install path label
    let mut install_path_label = fltk::frame::Frame::default()
        .with_size(750, 20)
        .below_of(&entrie_name, 5);
    install_path_label.set_label_size(20);
    install_path_label.set_label("Install Path");
    install_path_label.hide();
    install_path_label.set_label_color(Color::from_rgb(140, 140, 140));
    let install_path_label_arc = Arc::new(Mutex::new(install_path_label.clone()));

    // install path enable check
    let mut enable_install_path = fltk::button::Button::default()
        .with_size(20, 20)
        .below_of(&install_path_label, 1);
    enable_install_path.set_label("-");
    enable_install_path.hide();
    enable_install_path.set_color(Color::from_rgb(43, 43, 43));
    enable_install_path.set_label_color(Color::from_rgb(140, 140, 140));
    let enable_install_path_arc = Arc::new(Mutex::new(enable_install_path.clone()));

    // instal path input
    let mut install_path = fltk::input::Input::default()
        .with_size(665, 20)
        .right_of(&enable_install_path, 5);
    install_path.set_color(install_path.color().lighter());
    install_path.set_text_size(15);
    install_path.hide();
    install_path.set_color(Color::from_rgb(31, 31, 31));
    install_path.set_text_color(Color::from_rgb(140, 140, 140));
    let install_path_arc = Arc::new(Mutex::new(install_path.clone()));

    // check install path button
    let mut install_path_check_button = fltk::button::Button::default()
        .with_size(55, 20)
        .right_of(&install_path, 5);
    install_path_check_button.set_label("Check");
    install_path_check_button.hide();
    install_path_check_button.set_color(Color::from_rgb(43, 43, 43));
    install_path_check_button.set_label_color(Color::from_rgb(140, 140, 140));
    let install_path_check_button_arc = Arc::new(Mutex::new(install_path_check_button.clone()));

    // entry content
    let mut content_label = fltk::frame::Frame::default()
        .with_size(750, 20)
        .below_of(&enable_install_path, 5);
    content_label.set_label_size(20);
    content_label.set_label("Content");
    content_label.set_label_color(Color::from_rgb(140, 140, 140));
    content_label.hide();
    let content_label_arc = Arc::new(Mutex::new(content_label.clone()));
    let mut content = fltk::input::MultilineInput::default()
        .with_size(750, 150)
        .below_of(&content_label, 1);
    content.set_color(Color::from_rgb(31, 31, 31));
    content.set_text_color(Color::from_rgb(140, 140, 140));
    content.set_text_size(15);
    content.hide();
    let content_arc = Arc::new(Mutex::new(content.clone()));

    // clear content from the content box button (doesn't also save the entry)
    let mut clear_content_button = button::Button::default()
        .with_size(100, 20)
        .below_of(&content, 2);
    clear_content_button.set_label("Clear Content");
    clear_content_button.set_pos(225, clear_content_button.y());
    clear_content_button.set_color(Color::from_rgb(43, 43, 43));
    clear_content_button.set_label_color(Color::from_rgb(140, 140, 140));
    clear_content_button.hide();
    let clear_content_button_arc = Arc::new(Mutex::new(clear_content_button.clone()));

    // select file to add as entry button
    let mut select_file_button = button::Button::default()
        .with_size(100, 20)
        .below_of(&content, 2);
    select_file_button.set_label("Select File");
    select_file_button.set_pos(875, select_file_button.y());
    select_file_button.set_color(Color::from_rgb(43, 43, 43));
    select_file_button.set_label_color(Color::from_rgb(140, 140, 140));
    select_file_button.hide();
    let select_file_button_arc = Arc::new(Mutex::new(select_file_button.clone()));

    // entry notes
    let mut notes_label = fltk::frame::Frame::default()
        .with_size(750, 20)
        .below_of(&content, 5);
    notes_label.set_label_size(20);
    notes_label.set_label("Notes");
    notes_label.set_label_color(Color::from_rgb(140, 140, 140));
    notes_label.hide();
    let notes_label_arc = Arc::new(Mutex::new(notes_label.clone()));
    let mut notes = fltk::input::MultilineInput::default()
        .with_size(750, 150)
        .below_of(&notes_label, 1);
    notes.set_color(Color::from_rgb(31, 31, 31));
    notes.set_text_color(Color::from_rgb(140, 140, 140));
    notes.set_text_size(15);
    notes.hide();
    let notes_arc = Arc::new(Mutex::new(notes.clone()));

    // entry save button
    let mut save_button = fltk::button::Button::default()
        .with_size(75, 25)
        .below_of(&notes, 5);
    save_button.set_pos(900, save_button.y());
    save_button.set_label("Save");
    save_button.set_color(Color::from_rgb(43, 43, 43));
    save_button.set_label_color(Color::from_rgb(140, 140, 140));
    save_button.hide();
    let save_button_arc = Arc::new(Mutex::new(save_button.clone()));

    // delete entry button
    let mut delete_button = fltk::button::Button::default()
        .with_size(75, 25)
        .below_of(&notes, 5);
    delete_button.set_label("Delete");
    delete_button.set_color(Color::from_rgb(43, 43, 43));
    delete_button.set_label_color(Color::from_rgb(140, 140, 140));
    delete_button.hide();
    let delete_button_arc = Arc::new(Mutex::new(delete_button.clone()));

    // install entry button
    let mut install_button = fltk::button::Button::default()
        .with_size(75, 25)
        .below_of(&notes, 5);
    install_button.set_label("Install");
    install_button.set_pos(562, install_button.y());
    install_button.set_color(Color::from_rgb(43, 43, 43));
    install_button.set_label_color(Color::from_rgb(140, 140, 140));
    install_button.hide();
    let install_button_arc = Arc::new(Mutex::new(install_button.clone()));

    // status
    let mut status_label = fltk::frame::Frame::default()
        .with_size(750, 20)
        .below_of(&notes, 5);
    status_label.set_label_size(14);
    status_label.set_pos(status_label.x(), status_label.y() + 25);
    status_label.set_label_color(Color::from_rgb(140, 140, 140));
    status_label.hide();
    let status_label_arc = Arc::new(Mutex::new(status_label.clone()));

    // End customizing window
    wind.end();
    wind.make_resizable(true);

    // set window position and size same as start window
    wind.set_pos(login_wind.x(), login_wind.y());
    wind.set_size(login_wind.width(), login_wind.height());

    //
    // Window mechanics part
    //

    // create the object for encrypting
    let ecies = Arc::new(Mutex::new(ECIES::new()));

    // create a value to store the current selected entry in memory
    let current_selected_entry = Arc::new(Mutex::new(String::new()));

    // load vault
    let vault_db = super::core::load_vault(is_windows, &vault);

    // load current entries in db and display them
    // + save them in a dict where the key is the unecrypted value and the value is the one
    // encrypted

    let db_entries_dict =
        super::core::load_entries(&vault, vault_db.clone(), ecies.clone(), &mut entries);

    //
    //  Window callbacks
    //

    // clone the needed arc references
    let entrie_add_input_arc_clone = entrie_add_input_arc.clone();
    let entrie_name_arc_clone = entrie_name_arc.clone();
    let install_path_label_arc_clone = install_path_label_arc.clone();
    let install_path_arc_clone = install_path_arc.clone();
    let content_label_arc_clone = content_label_arc.clone();
    let content_arc_clone = content_arc.clone();
    let clear_content_button_arc_clone = clear_content_button_arc.clone();
    let select_file_button_arc_clone = select_file_button_arc.clone();
    let notes_label_arc_clone = notes_label_arc.clone();
    let notes_arc_clone = notes_arc.clone();
    let install_path_check_button_arc_clone = install_path_check_button_arc.clone();
    let status_label_arc_clone = status_label_arc.clone();
    let save_button_arc_clone = save_button_arc.clone();
    let delete_button_arc_clone = delete_button_arc.clone();
    let install_button_arc_clone = install_button_arc.clone();
    let enable_install_path_arc_clone = enable_install_path_arc.clone();
    // wind resize callback
    wind.resize_callback(move |_, _, _, w, h| {
        super::dry_callbacks::wind_resize_callback(
            w,
            h,
            entrie_add_input_arc_clone.clone(),
            entrie_add_button_arc.clone(),
            entrie_name_arc_clone.clone(),
            install_path_label_arc_clone.clone(),
            enable_install_path_arc_clone.clone(),
            install_path_arc_clone.clone(),
            install_path_check_button_arc_clone.clone(),
            content_label_arc_clone.clone(),
            content_arc_clone.clone(),
            clear_content_button_arc_clone.clone(),
            select_file_button_arc_clone.clone(),
            notes_label_arc_clone.clone(),
            notes_arc_clone.clone(),
            delete_button_arc_clone.clone(),
            install_button_arc_clone.clone(),
            save_button_arc_clone.clone(),
            status_label_arc_clone.clone(),
        );
    });

    // clone the needed arc references
    let entrie_name_arc_clone = entrie_name_arc.clone();
    let vault_arc_clone = Arc::new(Mutex::new(vault.clone()));
    let db_entries_dict_arc_clone = db_entries_dict.clone();
    let current_selected_entry_arc_clone = current_selected_entry.clone();
    let ecies_arc_clone = ecies.clone();
    let notes_label_arc_clone = notes_label_arc.clone();
    let notes_arc_clone = notes_arc.clone();
    let install_path_label_arc_clone = install_path_label_arc.clone();
    let install_path_arc_clone = install_path_arc.clone();
    let install_path_check_button_arc_clone = install_path_check_button_arc.clone();
    let content_label_arc_clone = content_label_arc.clone();
    let content_arc_clone = content_arc.clone();
    let clear_content_button_arc_clone = clear_content_button_arc.clone();
    let select_file_button_arc_clone = select_file_button_arc.clone();
    let vault_db_arc_clone = vault_db.clone();
    let status_label_arc_clone = status_label_arc.clone();
    let save_button_arc_clone = save_button_arc.clone();
    let delete_button_arc_clone = delete_button_arc.clone();
    let install_button_arc_clone = install_button_arc.clone();
    let enable_install_path_arc_clone = enable_install_path_arc.clone();
    // set entries callback
    entries.set_callback(move |e| {
        super::entries_callbacks::entries_callback(
            e,
            entrie_name_arc_clone.clone(),
            install_path_label_arc_clone.clone(),
            enable_install_path_arc_clone.clone(),
            install_path_arc_clone.clone(),
            install_path_check_button_arc_clone.clone(),
            content_label_arc_clone.clone(),
            content_arc_clone.clone(),
            clear_content_button_arc_clone.clone(),
            select_file_button_arc_clone.clone(),
            notes_label_arc_clone.clone(),
            notes_arc_clone.clone(),
            save_button_arc_clone.clone(),
            delete_button_arc_clone.clone(),
            install_button_arc_clone.clone(),
            status_label_arc_clone.clone(),
            current_selected_entry_arc_clone.clone(),
            db_entries_dict_arc_clone.clone(),
            vault_db_arc_clone.clone(),
            ecies_arc_clone.clone(),
            vault_arc_clone.clone(),
        );
    });

    // clone the needed arc references
    let entrie_add_input_arc_clone = entrie_add_input_arc.clone();
    let vault_arc_clone = Arc::new(Mutex::new(vault.clone()));
    let ecies_arc_clone = ecies.clone();
    let db_entries_dict_arc_clone = db_entries_dict.clone();
    let vault_db_arc_clone = vault_db.clone();
    let entries_arc_clone = entries_arc.clone();
    // set entrie add button callback
    entrie_add_button.set_callback(move |_| {
        super::entries_callbacks::entrie_add_button_callback(
            entrie_add_input_arc_clone.clone(),
            vault_db_arc_clone.clone(),
            vault_arc_clone.clone(),
            ecies_arc_clone.clone(),
            db_entries_dict_arc_clone.clone(),
            entries_arc_clone.clone(),
        );
    });

    // clone the needed arc references
    let vault_arc_clone = Arc::new(Mutex::new(vault.clone()));
    let current_selected_entry_arc_clone = current_selected_entry.clone();
    let db_entries_dict_arc_clone = db_entries_dict.clone();
    let ecies_arc_clone = ecies.clone();
    let vault_db_arc_clone = vault_db.clone();
    let notes_arc_clone = notes_arc.clone();
    let install_path_arc_clone = install_path_arc.clone();
    let content_arc_clone = content_arc.clone();
    let status_label_arc_clone = status_label_arc.clone();
    // set save button callback
    save_button.set_callback(move |_| {
        super::value_callbacks::save_button_callback(
            install_path_arc_clone.clone(),
            content_arc_clone.clone(),
            notes_arc_clone.clone(),
            status_label_arc_clone.clone(),
            current_selected_entry_arc_clone.clone(),
            vault_db_arc_clone.clone(),
            vault_arc_clone.clone(),
            db_entries_dict_arc_clone.clone(),
            ecies_arc_clone.clone(),
        );
    });

    // clone the needed arc references
    let current_selected_entry_arc_clone = current_selected_entry.clone();
    let db_entries_dict_arc_clone = db_entries_dict.clone();
    let entries_arc_clone = entries_arc.clone();
    let vault_db_arc_clone = vault_db.clone();
    // set delete button callback
    delete_button.set_callback(move |_| {
        super::value_callbacks::delete_button_callback(
            vault_db_arc_clone.clone(),
            current_selected_entry_arc_clone.clone(),
            db_entries_dict_arc_clone.clone(),
            entries_arc_clone.clone(),
        );
    });

    // clone the needed arc references
    let install_path_arc_clone = install_path_arc.clone();
    let content_arc_clone = content_arc.clone();
    let status_label_arc_clone = status_label_arc.clone();
    let vault_arc_clone = Arc::new(Mutex::new(vault.clone()));
    let db_entries_dict_arc_clone = db_entries_dict.clone();
    let ecies_arc_clone = ecies.clone();
    let current_selected_entry_arc_clone = current_selected_entry.clone();
    let vault_db_arc_clone = vault_db.clone();
    // set install button callback
    install_button.set_callback(move |_| {
        super::value_callbacks::install_button_callback(
            install_path_arc_clone.clone(),
            content_arc_clone.clone(),
            vault_arc_clone.clone(),
            vault_db_arc_clone.clone(),
            ecies_arc_clone.clone(),
            current_selected_entry_arc_clone.clone(),
            db_entries_dict_arc_clone.clone(),
            status_label_arc_clone.clone(),
            is_windows,
        );
    });

    // clone the needed arc references
    let install_path_arc_clone = install_path_arc.clone();
    let status_label_arc_clone = status_label_arc.clone();
    // set install path check button callback
    install_path_check_button.set_callback(move |_| {
        super::dry_callbacks::install_path_check_button_callback(
            status_label_arc_clone.clone(),
            install_path_arc_clone.clone(),
        );
    });

    // clone the needed arc references
    let content_arc_clone = content_arc.clone();
    let status_label_arc_clone = status_label_arc.clone();
    let current_selected_entry_arc_clone = current_selected_entry.clone();
    let vault_arc_clone = Arc::new(Mutex::new(vault.clone()));
    let vault_db_arc_clone = vault_db.clone();
    let ecies_arc_clone = ecies.clone();
    let db_entries_dict_arc_clone = db_entries_dict.clone();
    // set select file button callback
    select_file_button.set_callback(move |_| {
        super::value_callbacks::select_file_button(
            status_label_arc_clone.clone(),
            content_arc_clone.clone(),
            current_selected_entry_arc_clone.clone(),
            vault_db_arc_clone.clone(),
            ecies_arc_clone.clone(),
            vault_arc_clone.clone(),
            db_entries_dict_arc_clone.clone(),
        );
    });

    // clone the needed arc references
    let status_label_arc_clone = status_label_arc.clone();
    let content_arc_clone = content_arc.clone();
    // set delete content button callback
    clear_content_button.set_callback(move |_| {
        super::dry_callbacks::clear_content_button_callback(
            content_arc_clone.clone(),
            status_label_arc_clone.clone(),
        );
    });

    // clone the needed arc references
    let install_path_arc_clone = install_path_arc.clone();
    // set enable/disable install path button
    enable_install_path.set_callback(move |b| {
        super::dry_callbacks::enable_install_path_button_callback(
            b,
            install_path_arc_clone.clone(),
        );
    });

    wind
}
