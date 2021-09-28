use fltk::{prelude::*, enums::*, *};
use fltk_theme::{WidgetScheme, SchemeType};
use fltk_theme::widget_schemes::aqua::dark::*; // get all the dark aqua colors

fn main() {
    let a = app::App::default();
    app::background(BG_COL.0, BG_COL.1, BG_COL.2);
    app::background2(CTRL_ACC_COL.0, CTRL_ACC_COL.1, CTRL_ACC_COL.2);
    app::foreground(FG_COL.0, FG_COL.1, FG_COL.2);
    app::set_color(Color::Selection, 255, 255, 255);
    let widget_scheme = WidgetScheme::new(SchemeType::Aqua);
    widget_scheme.apply();
    let mut win = window::Window::default().with_size(400, 300);
    let mut inp = input::Input::new(50, 50, 300, 30, None);
    inp.set_color(Color::from_tup(*FRAME_COL));
    let mut check = button::CheckButton::new(160, 150, 80, 30, "  Check");
    check.set_value(true);
    check.set_frame(enums::FrameType::FlatBox);
    let mut round = button::RoundButton::new(160, 180, 80, 30, "  Round");
    round.set_value(true);
    round.set_frame(enums::FrameType::FlatBox);
    let mut btn = button::Button::new(160, 230, 80, 30, "Hello");
    btn.set_color(Color::from_tup(*CTRL_COL));
    btn.set_selection_color(Color::from_tup(*SYS_CYAN));
    win.end();
    win.make_resizable(true);
    win.show();
    a.run().unwrap();
}
