use fltk::{enums::*, prelude::*, *};
use fltk_theme::widget_schemes::fluent::colors::*;
use fltk_theme::widget_schemes::fluent::frames::*;
use fltk_theme::{SchemeType, WidgetScheme};

fn main() {
    let a = app::App::default();
    app::set_font_size(12);
    app::background(0xFC, 0xFC, 0xFC);
    app::background2(0xFF, 0xFF, 0xFF);
    app::foreground(0x00, 0x00, 0x00);
    app::set_color(
        Color::Selection,
        SELECTION_COLOR.0,
        SELECTION_COLOR.1,
        SELECTION_COLOR.2,
    );
    let theme = WidgetScheme::new(SchemeType::Fluent);
    theme.apply();
    let mut win = window::SingleWindow::default()
        .with_size(400, 300)
        .with_label("Fluent");
    let mut col = group::Flex::default()
        .with_size(340, 240)
        .center_of_parent()
        .column();
    col.set_frame(FrameType::EngravedBox);
    col.set_margins(100, 60, 100, 60);
    let mut choice = menu::Choice::default();
    choice.add_choice("Fluent");
    choice.set_value(0);
    choice.set_frame(FrameType::FlatBox);
    let mut check = button::CheckButton::default().with_label("  Check");
    check.set_value(true);
    check.set_frame(FrameType::FlatBox);
    let mut round = button::RoundButton::default().with_label("  Round");
    round.set_value(true);
    round.set_frame(FrameType::FlatBox);
    let mut toggle = button::ToggleButton::default().with_label("Toggle");
    toggle.set_color(Color::from_rgba_tuple(ACCENT_COLOR));
    toggle.set_label_color(Color::White);
    toggle.set_selection_color(toggle.color().darker());
    let mut btn = button::Button::default().with_label("Hello");
    btn.set_frame(OS_DEFAULT_BUTTON_UP_BOX);
    btn.set_down_frame(OS_DEFAULT_DEPRESSED_DOWN_BOX);
    // handle hover
    btn.handle(|b, ev| match ev {
        Event::Enter => {
            b.set_frame(OS_HOVERED_UP_BOX);
            b.redraw();
            true
        }
        Event::Leave => {
            b.set_frame(OS_DEFAULT_BUTTON_UP_BOX);
            b.redraw();
            true
        }
        _ => false,
    });
    col.end();
    win.end();
    win.make_resizable(true);
    win.show();
    a.run().unwrap();
}
