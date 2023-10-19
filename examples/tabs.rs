use fltk::{prelude::*, *, group::*};
use fltk_theme::{SchemeType, WidgetScheme};

fn main() {
    let a = app::App::default();
    
    let scheme = WidgetScheme::new(SchemeType::Gleam);
    scheme.apply();
    
    let mut win = window::Window::default().with_size(400, 300);
    win.make_resizable(true);
    

    let mut tabs = Tabs::default_fill();
    
    let t1 = Flex::default().with_label("Hello this is a very long string");
    let mut choice = menu::Choice::new(100, 100, 200, 30, None);
    choice.add_choice("Clean|Crystal|Gleam");
    choice.set_value(2);
    t1.end();

    let t2 = Flex::default().with_label("Another long string!!!");
    t2.end();

    let t3 = Flex::default().with_label("Third tab to show you what's wrong");
    t3.end();
    
    tabs.end();
    tabs.auto_layout();
    tabs.show();
    
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
