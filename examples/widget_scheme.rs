use fltk::{enums::*, prelude::*, *};
use fltk_theme::{SchemeType, WidgetScheme};

fn main() {
    let a = app::App::default();
    let scheme = WidgetScheme::new(SchemeType::Clean);
    scheme.apply();
    let mut win = window::Window::default()
        .with_size(400, 300)
        .with_label("Widget Scheme");
    let mut col = group::Flex::default()
        .with_size(340, 240)
        .center_of_parent()
        .column();
    col.set_frame(FrameType::EngravedBox);
    col.set_margins(100, 60, 100, 60);
    let mut choice = menu::Choice::default();
    choice.add_choice("Clean|Crystal|Gleam");
    choice.set_value(3);
    let mut check = button::CheckButton::default().with_label("Check");
    check.set_value(true);
    let mut round = button::RoundButton::default().with_label("Round");
    round.set_value(true);
    let mut _btn = button::Button::default().with_label("Hello");
    col.end();
    win.end();
    win.show();
    choice.set_callback(|c| {
        let scheme = match c.value() {
            0 => WidgetScheme::new(SchemeType::Clean),
            1 => WidgetScheme::new(SchemeType::Crystal),
            2 => WidgetScheme::new(SchemeType::Gleam),
            _ => unimplemented!(),
        };
        scheme.apply();
    });
    a.run().unwrap();
}
