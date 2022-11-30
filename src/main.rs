mod encrypter;
mod hasher;
mod login;
mod signup;
mod start;
mod utils;
mod vault;
use encrypter::ecies::ECIES;
use fltk::{
    app,
    prelude::*,
};
use fltk_theme::{color_themes, ColorTheme, SchemeType, WidgetScheme};

fn main() {
    // Configure app and theme it
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let theme = ColorTheme::new(color_themes::BLACK_THEME);
    theme.apply();
    let widget_scheme = WidgetScheme::new(SchemeType::Clean);
    widget_scheme.apply();

    // Create signup window
    // let mut signup_wind = signup::window::create(utils::is_windows());
    // signup_wind.show();

    // Create login window
    // let mut login_wind = login::window::create(utils::is_windows());
    // login_wind.show();

    // Create vault window
    // let mut vault_wind = vault::window::create(utils::is_windows(), Vault::new_empty());
    // vault_wind.show();

    // Create start window
    let mut start_wind = start::window::create();
    start_wind.show();

    // Start the app
    app.run().unwrap();
}
