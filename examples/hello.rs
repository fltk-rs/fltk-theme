use fltk::{prelude::*, *};
use fltk_theme::{ColorTheme, color_themes};

fn main() {
    let a = app::App::default().with_scheme(app::Scheme::Gtk);
    let theme = ColorTheme::from_colormap(color_themes::BLACK_THEME);
    theme.apply();
    let mut win = window::Window::default().with_size(400, 300);
    let mut btn = button::Button::new(160, 200, 80, 40, "Hello");
    btn.set_color(btn.color().lighter());
    win.end();
    win.show();
    a.run().unwrap();
}
