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

    // value in entrie
    let mut entrie_name = fltk::text::TextDisplay::default().with_size(700, 25).right_of(&entries, 50);
    entrie_name.set_color(entrie_name.color().lighter());
    entrie_name.set_text_size(15);
    entrie_name.set_pos(entrie_name.x(), 20);
    
    // End customizing window
    // flex.end();
    wind.end();

    // Window callbacks
    entries.set_callback(move |e| {
        let mut entrie_name_buf = text::TextBuffer::default();
        // TODO: handle error
        let selected_item = &e.first_selected_item().unwrap().label().unwrap();
        entrie_name_buf.set_text(selected_item);
        // let insert_pos = (selected_item.len() as f64 / 2.0).floor() as i32;
        entrie_name.set_buffer(entrie_name_buf.clone());    
    });
    
    wind
}
