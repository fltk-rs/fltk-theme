use fltk::{prelude::*, *};
use fltk_theme::{color_themes, ColorTheme};

fn main() {
    let a = app::App::default().with_scheme(app::Scheme::Gtk);
    app::set_visible_focus(false);

    let color_theme = ColorTheme::from_colormap(color_themes::BLACK_THEME);
    color_theme.apply();
    let mut win = window::Window::default().with_size(400, 300);
    let mut choice = menu::Choice::new(100, 100, 200, 30, None);
    choice.add_choice("Black|Dark|Gray|Shake|Tan");
    choice.set_value(0);
    let mut check = button::CheckButton::new(160, 150, 80, 30, "  Check");
    check.set_value(true);
    check.set_frame(enums::FrameType::FlatBox);
    let mut round = button::RoundButton::new(160, 180, 80, 30, "  Round");
    round.set_value(true);
    round.set_frame(enums::FrameType::FlatBox);
    button::Button::new(160, 220, 80, 30, "Hello");
    win.end();
    win.show();
    choice.set_callback(|c| {
        let theme = match c.value() {
            0 => ColorTheme::from_colormap(color_themes::BLACK_THEME),
            1 => ColorTheme::from_colormap(color_themes::DARK_THEME),
            2 => ColorTheme::from_colormap(color_themes::GRAY_THEME),
            3 => ColorTheme::from_colormap(color_themes::SHAKE_THEME),
            4 => ColorTheme::from_colormap(color_themes::TAN_THEME),
            _ => ColorTheme::from_colormap(color_themes::BLACK_THEME),
        };
        theme.apply();
    });

    a.run().unwrap();
}
