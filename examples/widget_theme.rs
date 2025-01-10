use fltk::{enums::*, prelude::*, *};
use fltk_theme::{widget_themes, ThemeType, WidgetTheme};

fn main() {
    let a = app::App::default();
    let theme = WidgetTheme::new(ThemeType::AquaClassic);
    theme.apply();
    let mut win = window::Window::default()
        .with_size(400, 300)
        .with_label("Widget Theme");
    let mut col = group::Flex::default()
        .with_size(340, 240)
        .center_of_parent()
        .column();
    col.set_frame(FrameType::EngravedBox);
    col.set_margins(100, 60, 100, 60);
    let mut choice = menu::Choice::default();
    choice.add_choice("Classic|Aero|Metro|AquaClassic|Greybird|Blue|HighContrast|Dark");
    choice.set_value(3);
    choice.set_frame(widget_themes::OS_PANEL_THIN_UP_BOX);
    let mut check = button::CheckButton::default().with_label("  Check");
    check.set_value(true);
    check.set_frame(enums::FrameType::FlatBox);
    let mut round = button::RoundButton::default().with_label("  Round");
    round.set_value(true);
    round.set_frame(enums::FrameType::FlatBox);
    let mut btn = button::Button::default().with_label("Hello");
    btn.set_frame(widget_themes::OS_DEFAULT_BUTTON_UP_BOX);
    col.end();
    win.end();
    win.show();
    choice.set_callback(|c| {
        let theme = match c.value() {
            0 => WidgetTheme::new(ThemeType::Classic),
            1 => WidgetTheme::new(ThemeType::Aero),
            2 => WidgetTheme::new(ThemeType::Metro),
            3 => WidgetTheme::new(ThemeType::AquaClassic),
            4 => WidgetTheme::new(ThemeType::Greybird),
            5 => WidgetTheme::new(ThemeType::Blue),
            6 => WidgetTheme::new(ThemeType::HighContrast),
            7 => WidgetTheme::new(ThemeType::Dark),
            _ => WidgetTheme::new(ThemeType::Classic),
        };
        theme.apply();
    });

    a.run().unwrap();
}
