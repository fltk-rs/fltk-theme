use fltk::{prelude::*, *};
use fltk_theme::{WidgetTheme, WidgetThemeType};

fn main() {
    let a = app::App::default();
    let widget_theme = WidgetTheme::new(WidgetThemeType::Classic);
    widget_theme.apply();
    let mut win = window::Window::default().with_size(400, 300);
    let mut _btn = button::Button::new(160, 200, 80, 40, "Hello");
    win.end();
    win.show();
    a.run().unwrap();
}