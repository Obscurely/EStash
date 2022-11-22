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

pub fn create(is_windows: bool, vault: Vault) -> fltk::window::DoubleWindow {
    // Create vault window
    let mut wind = Window::default().with_size(1000, 500).with_label("Vault");
    // let mut flex = Flex::default()
    //     .with_size(950, 450)
    //     .center_of_parent()
    //     .column();

    // entries coloumn
    let mut entries = tree::Tree::default().with_size(200, 500);
    entries.add("Test1");
    entries.add("Test2");
    entries.add("Test3");
    entries.add("Test4");
    entries.add("Test5");

    // create a grid for the content inside the entrie
    // let mut grid = Grid::default_fill();
    // grid.debug(true); // set to true to show cell outlines and numbers
    // grid.set_layout(5, 1); // 5 rows, 5 columns

    // values in entrie
    // let mut entrie_name = fltk::text::TextDisplay::default().with_size(700, 25).right_of(&entries, 50);
    // entrie_name.set_color(entrie_name.color().lighter());
    // entrie_name.set_text_size(15);
    // entrie_name.set_pos(entrie_name.x(), 20);

    let mut entrie_name = fltk::frame::Frame::default().with_size(700, 25).right_of(&entries, 50);
    entrie_name.set_label_size(30);
    entrie_name.set_pos(entrie_name.x(), 20);
    entrie_name.hide();

    let mut install_path_label = fltk::frame::Frame::default().with_size(700, 20).below_of(&entrie_name, 15);
    install_path_label.set_label_size(20);
    install_path_label.set_label("Install Path");
    install_path_label.hide();
    let mut install_path = fltk::input::Input::default().with_size(700, 20).below_of(&install_path_label, 1);
    install_path.set_color(install_path.color().lighter());
    install_path.set_text_size(15);
    install_path.hide();

    let mut content_label = fltk::frame::Frame::default().with_size(700, 20).below_of(&install_path, 15);
    content_label.set_label_size(20);
    content_label.set_label("Content");
    content_label.hide();
    let mut content = fltk::input::MultilineInput::default().with_size(700, 150).below_of(&content_label, 1);
    content.set_color(content.color().lighter());
    content.set_text_size(15);
    content.hide();

    let mut notes_label = fltk::frame::Frame::default().with_size(700, 20).below_of(&content, 15);
    notes_label.set_label_size(20);
    notes_label.set_label("Notes");
    notes_label.hide();
    let mut notes = fltk::input::MultilineInput::default().with_size(700, 150).below_of(&notes_label, 1);
    notes.set_color(notes.color().lighter());
    notes.set_text_size(15);
    notes.hide();

    // End customizing window
    // flex.end();
    wind.end();

    // Window callbacks
    entries.set_callback(move |e| {
        let selected_item = &e.first_selected_item().unwrap().label().unwrap();

        if selected_item == "ROOT" {
            entrie_name.hide();
            install_path_label.hide();
            install_path.hide();
            content_label.hide();
            content.hide();
            notes_label.hide();
            notes.hide();
            panic!(); // using panic to leave callback, because of fltk's nature this works with no
                      // problem
        }

        // TODO: load entry
        // TODO: set values

        // Unhide widgets
        entrie_name.show();
        install_path_label.show();
        install_path.show();
        content_label.show();
        content.show();
        notes_label.show();
        notes.show();
        // TODO: handle error
        entrie_name.set_label(selected_item);
    });
    
    wind
}
