mod encrypter;
mod hasher;
mod login;
mod signup;
mod start;
mod utils;
mod vault;
use encrypter::ecies::ECIES;
use fltk::{app, prelude::*};
use fltk_theme::{color_themes, ColorTheme, SchemeType, WidgetScheme};

fn main() {
    // Configure app and theme it
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let theme = ColorTheme::new(color_themes::BLACK_THEME);
    theme.apply();
    let widget_scheme = WidgetScheme::new(SchemeType::Clean);
    widget_scheme.apply();

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
            drop(widget_scheme);
            drop(theme);
            drop(app);

            // exit the program
            std::process::exit(100);
        }
    };
}
