use fltk::{prelude::*, *};
use fltk_theme::{widget_themes, ThemeType, WidgetTheme};

fn main() {
    let a = app::App::default();
    let theme = WidgetTheme::new(ThemeType::Fluent);
    theme.apply();
    let mut win = window::Window::default().with_size(400, 300);
    let mut choice = menu::Choice::new(100, 100, 200, 30, None);
    choice.add_choice("Fluent");
    choice.set_value(0);
    choice.set_frame(widget_themes::OS_PANEL_THIN_UP_BOX);
    let mut check = button::CheckButton::new(160, 150, 80, 30, "  Check");
    check.set_value(true);
    let mut round = button::RoundButton::new(160, 180, 80, 30, "  Round");
    round.set_value(true);
    let mut toggle = button::ToggleButton::new(100, 220, 80, 30, "Toggle");
    toggle.set_color(enums::Color::from_hex(0x0078D4));
    toggle.set_label_color(enums::Color::White);
    toggle.set_down_frame(widget_themes::OS_DEFAULT_DEPRESSED_DOWN_BOX);
    let mut btn = button::Button::new(220, 220, 80, 30, "Hello");
    btn.set_frame(widget_themes::OS_DEFAULT_BUTTON_UP_BOX);
    btn.set_down_frame(widget_themes::OS_DEFAULT_DEPRESSED_DOWN_BOX);
    btn.handle(|b, ev| match ev {
        enums::Event::Enter => {
            b.set_frame(widget_themes::OS_HOVERED_UP_BOX);
            b.redraw();
            true
        },
        enums::Event::Leave => {
            b.set_frame(widget_themes::OS_DEFAULT_BUTTON_UP_BOX);
            b.redraw();
            true
        },
        _ => false,
    });
    win.end();
    win.show();
    a.run().unwrap();
}
