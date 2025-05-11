use fltk::{enums::*, prelude::*, *};
use fltk_theme::{color_themes, reset_color_map, ColorTheme};

fn main() {
    let a = app::App::default().with_scheme(app::Scheme::Gtk);
    app::set_visible_focus(false);
    let color_theme = ColorTheme::new(color_themes::BLACK_THEME);
    color_theme.apply();
    let mut win = window::Window::default()
        .with_size(400, 300)
        .with_label("Color Theme");
    let mut col = group::Flex::default()
        .with_size(340, 240)
        .center_of_parent()
        .column();
    col.set_frame(FrameType::EngravedBox);
    col.set_margins(100, 60, 100, 60);
    let mut choice = menu::Choice::default();
    choice.add_choice("Black|Dark|Gray|Shake|Tan");
    choice.set_value(0);
    let mut check = button::CheckButton::default().with_label("  Check");
    check.set_value(true);
    check.set_frame(enums::FrameType::FlatBox);
    let mut round = button::RoundButton::default().with_label("  Round");
    round.set_value(true);
    round.set_frame(enums::FrameType::FlatBox);
    let mut button = button::Button::default().with_label("Hello");
    col.end();
    win.end();
    win.show();
    choice.set_callback(|c| {
        reset_color_map();
        let theme = match c.value() {
            0 => ColorTheme::new(color_themes::BLACK_THEME),
            1 => ColorTheme::new(color_themes::DARK_THEME),
            2 => ColorTheme::new(color_themes::GRAY_THEME),
            3 => ColorTheme::new(color_themes::SHAKE_THEME),
            4 => ColorTheme::new(color_themes::TAN_THEME),
            _ => ColorTheme::new(color_themes::BLACK_THEME),
        };
        theme.apply();
    });
    button.set_callback(move |_| {
        choice.set_value(-1);
        reset_color_map();
    });
    a.run().unwrap();
}
