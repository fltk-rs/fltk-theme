use fltk::{enums::*, prelude::*, *};
use fltk_theme::widget_schemes::fluent::colors::*;
use fltk_theme::{SchemeType, WidgetScheme};

fn main() {
    let a = app::App::default();
    app::set_font_size(12);
    app::background(0x00, 0x00, 0x00);
    app::background2(0x00, 0x00, 0x00);
    app::foreground(0xff, 0xff, 0xff);
    app::set_color(
        Color::Selection,
        SELECTION_COLOR.0,
        SELECTION_COLOR.1,
        SELECTION_COLOR.2,
    );
    let scheme = WidgetScheme::new(SchemeType::Fluent);
    scheme.apply();
    let mut win = window::Window::default()
        .with_size(400, 300)
        .with_label("Widget Scheme");
    let mut col = group::Flex::default()
        .with_size(340, 240)
        .center_of_parent()
        .column();
    col.set_frame(FrameType::EngravedBox);
    // col.set_margins(100, 60, 100, 60);
    let mut term = text::SimpleTerminal::default();
    term.set_stay_at_bottom(false);
    term.set_text(&std::fs::read_to_string("README.md").unwrap());
    // let mut choice = menu::Choice::default();
    // choice.add_choice("Clean|Crystal|Gleam");
    // choice.set_value(3);
    // let mut check = button::CheckButton::default().with_label("Check");
    // check.set_value(true);
    // let mut round = button::RoundButton::default().with_label("Round");
    // round.set_value(true);
    // let mut _btn = button::Button::default().with_label("Hello");
    col.end();
    win.end();
    win.show();
    // choice.set_callback(|c| {
    //     let scheme = match c.value() {
    //         0 => WidgetScheme::new(SchemeType::Clean),
    //         1 => WidgetScheme::new(SchemeType::Crystal),
    //         2 => WidgetScheme::new(SchemeType::Gleam),
    //         _ => unimplemented!(),
    //     };
    //     scheme.apply();
    // });
    a.run().unwrap();
}
