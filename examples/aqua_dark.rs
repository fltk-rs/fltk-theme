use fltk::{prelude::*, enums::*, *};
use fltk_theme::{WidgetScheme, SchemeType};
use fltk_theme::widget_schemes::aqua::frames::*;

use fltk_theme::widget_schemes::aqua::dark::*; // get all the dark aqua colors
// use fltk_theme::widget_schemes::aqua::light::*; // get all the light aqua colors
// use fltk_theme::widget_schemes::aqua::sys::*; // get all the system aqua colors, requires MacOS

fn main() {
    let a = app::App::default();
    app::background(windowBackgroundColor.0, windowBackgroundColor.1, windowBackgroundColor.2);
    app::background2(controlAccentColor.0, controlAccentColor.1, controlAccentColor.2);
    app::foreground(labelColor.0, labelColor.1, labelColor.2);
    app::set_color(Color::Selection, 255, 255, 255);
    let widget_scheme = WidgetScheme::new(SchemeType::Aqua);
    widget_scheme.apply();
    let mut win = window::Window::default().with_size(400, 300);
    let mut choice = menu::Choice::new(100, 50, 200, 30, None);
    choice.set_color(Color::from_rgba(*controlColor));
    choice.add_choice("Opt1|Opt2|Opt3");
    choice.set_value(2);
    let mut inp = input::Input::new(50, 100, 300, 30, None);
    inp.set_color(Color::from_rgba(*controlColor));
    let mut check = button::CheckButton::new(160, 150, 80, 30, "  Check");
    check.set_value(true);
    check.set_frame(enums::FrameType::FlatBox);
    let mut round = button::RoundButton::new(160, 180, 80, 30, "  Round");
    round.set_value(true);
    round.set_frame(enums::FrameType::FlatBox);
    let mut btn = button::Button::new(160, 230, 80, 30, "Hello");
    btn.set_color(Color::from_rgba(*controlColor));
    btn.set_selection_color(Color::from_rgba(*controlAccentColor));
    btn.set_frame(OS_DEFAULT_BUTTON_UP_BOX);
    win.end();
    win.make_resizable(true);
    win.show();
    a.run().unwrap();
}
