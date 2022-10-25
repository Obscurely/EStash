use fltk::{app, button::Button, frame::Frame, group::Flex, prelude::*, window::Window, enums::{Font, Align}};
use fltk_theme::{ColorTheme, color_themes, WidgetScheme, SchemeType};
fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let theme = ColorTheme::new(color_themes::BLACK_THEME);
    theme.apply();
    let widget_scheme = WidgetScheme::new(SchemeType::Clean);
    widget_scheme.apply();
    let mut wind = Window::default().with_size(500, 200).with_label("Counter");
    let mut flex = Flex::default().with_size(460, 160).center_of_parent().column();
    let mut input_user = fltk::input::Input::default();
    let mut input_pass = fltk::input::Input::default();
    let mut but_login = Button::default().with_label("Login");
    input_user.set_color(input_user.color().lighter());
    input_user.set_text_size(20);
    input_pass.set_color(input_pass.color().lighter());
    input_pass.set_text_size(20);
    but_login.set_color(but_login.color().lighter());
    flex.end();
    wind.end();
    wind.show();
    // but_dec.set_callback(move |btn| {
    //     println!("your input is {}", user_input.value());
    // });
    app.run().unwrap();
}
