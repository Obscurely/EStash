#![windows_subsystem = "windows"] // hide terminal
mod encrypter;
mod hasher;
mod login;
mod signup;
mod start;
mod utils;
mod vault;
use encrypter::ecies::ECIES;
use fltk::{app, prelude::*, enums::FrameType};

fn main() {
    // Configure app and theme it
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    // let theme = ColorTheme::new(color_themes::BLACK_THEME);
    // theme.apply();
    // let widget_scheme = WidgetScheme::new(SchemeType::Clean);
    // widget_scheme.apply();

    fltk::app::background(37, 37, 37);
    fltk::app::set_selection_color(112, 112, 112);
    fltk::app::set_frame_type(FrameType::GtkDownBox);
    fltk::app::set_visible_focus(false);

    // Create start window
    let mut start_wind = start::window::create();
    start_wind.show();

    // Start the app
    match app.run() {
        Ok(_) => (),
        Err(err) => {
            // print error
            eprintln!("ERROR: Failed to start EStash, given error:\n{err}");

            // drop the objects manually, cause why not
            drop(start_wind);
            // drop(widget_scheme);
            // drop(theme);
            drop(app);

            // exit the program
            std::process::exit(100);
        }
    };
}
