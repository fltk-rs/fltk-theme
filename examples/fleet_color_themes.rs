use fltk::{enums::*, prelude::*, *};
use fltk_theme::{color_themes, reset_color_map, ColorMap, ColorTheme};
use fltk_theme::{SchemeType, WidgetScheme};

const FLEET_THEMES: &[&[ColorMap]] = &[
    color_themes::fleet::LIGHT,
    color_themes::fleet::DARK1,
    color_themes::fleet::DARK2,
    color_themes::fleet::TAN,
    color_themes::fleet::DARK_TAN,
    color_themes::fleet::MARINE,
    color_themes::fleet::BLUEISH,
    color_themes::fleet::NORD,
    color_themes::fleet::HIGH_CONTRAST,
    color_themes::fleet::FOREST,
    color_themes::fleet::PURPLE_DUSK,
    color_themes::fleet::SOLARIZED_LIGHT,
    color_themes::fleet::SOLARIZED_DARK,
    color_themes::fleet::MONOKAI,
    color_themes::fleet::GRUVBOX_LIGHT,
    color_themes::fleet::GRUVBOX_DARK,
    color_themes::fleet::DRACULA,
    color_themes::fleet::OCEANIC_NEXT,
    color_themes::fleet::MINIMALIST,
    color_themes::fleet::AUTUMN,
    color_themes::fleet::CYBERPUNK,
    color_themes::fleet::MATERIAL_DARK,
    color_themes::fleet::MINT,
    color_themes::fleet::VINTAGE,
    color_themes::fleet::GRAY,
];

fn main() {
    let a = app::App::default().with_scheme(app::Scheme::Gtk);
    app::set_visible_focus(false);
    let color_theme = ColorTheme::new(color_themes::fleet::LIGHT);
    color_theme.apply();
    let mut win = window::Window::default()
        .with_size(400, 300)
        .with_label("Color Theme");
    let mut col = group::Flex::default()
        .with_size(340, 240)
        .center_of_parent()
        .column();
    col.set_frame(FrameType::EngravedBox);
    col.set_margins(80, 40, 80, 40);
    let mut choice = menu::Choice::default();
    choice.add_choice(
        "LIGHT|DARK1|DARK2|TAN|DARK_TAN|MARINE|BLUEISH|NORD|HIGH_CONTRAST|FOREST|PURPLE_DUSK|SOLARIZED_LIGHT|SOLARIZED_DARK|MONOKAI|GRUVBOX_LIGHT|GRUVBOX_DARK|DRACULA|OCEANIC_NEXT|MINIMALIST|AUTUMN|CYBERPUNK|MATERIAL_DARK|MINT|VINTAGE|GRAY"
    );
    choice.set_value(0);
    let mut inp = input::Input::default();
    inp.set_value("Sample text!");
    let mut check = button::CheckButton::default().with_label("  Check");
    check.set_value(true);
    check.set_frame(enums::FrameType::FlatBox);
    let mut round = button::RoundButton::default().with_label("  Round");
    round.set_value(true);
    round.set_frame(enums::FrameType::FlatBox);
    let mut button = button::Button::default().with_label("Hello");
    button.set_callback(|_| reset_color_map());
    col.end();
    win.end();
    win.show();
    choice.set_callback(|c| {
        let theme = ColorTheme::new(FLEET_THEMES[c.value() as usize]);
        theme.apply();
    });

    a.run().unwrap();
}
