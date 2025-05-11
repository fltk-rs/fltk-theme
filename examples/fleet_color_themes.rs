use fltk::{prelude::*, *};
use fltk_theme::{color_themes, reset_color_map, ColorTheme};

fn main() {
    let themes = [
        color_themes::fleet::LIGHT.clone(),
        color_themes::fleet::DARK1.clone(),
        color_themes::fleet::DARK2.clone(),
        color_themes::fleet::TAN.clone(),
        color_themes::fleet::DARK_TAN.clone(),
        color_themes::fleet::MARINE.clone(),
        color_themes::fleet::BLUEISH.clone(),
        color_themes::fleet::NORD.clone(),
        color_themes::fleet::HIGH_CONTRAST.clone(),
        color_themes::fleet::FOREST.clone(),
        color_themes::fleet::PURPLE_DUSK.clone(),
        color_themes::fleet::SOLARIZED_LIGHT.clone(),
        color_themes::fleet::SOLARIZED_DARK.clone(),
        color_themes::fleet::MONOKAI.clone(),
        color_themes::fleet::GRUVBOX_LIGHT.clone(),
        color_themes::fleet::GRUVBOX_DARK.clone(),
        color_themes::fleet::DRACULA.clone(),
        color_themes::fleet::OCEANIC_NEXT.clone(),
        color_themes::fleet::MINIMALIST.clone(),
        color_themes::fleet::AUTUMN.clone(),
        color_themes::fleet::CYBERPUNK.clone(),
        color_themes::fleet::MATERIAL_DARK.clone(),
        color_themes::fleet::MINT.clone(),
        color_themes::fleet::VINTAGE.clone(),
        color_themes::fleet::GRAY.clone(),
    ];
    let a = app::App::default().with_scheme(app::Scheme::Gtk);
    app::set_visible_focus(false);
    let color_theme = ColorTheme::new(&*color_themes::fleet::LIGHT);
    color_theme.apply();
    let mut win = window::Window::default()
        .with_size(400, 300)
        .with_label("Fleet Color Theme");
    let mut col = group::Flex::default()
        .with_size(340, 240)
        .center_of_parent()
        .column();
    col.set_margins(80, 40, 80, 40);
    col.set_frame(enums::FrameType::EngravedBox);
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
    let mut button = button::Button::default().with_label("Reset");
    col.end();
    win.end();
    win.show();
    choice.set_callback(move |c| {
        reset_color_map();
        let theme = ColorTheme::new(&themes[c.value() as usize]);
        theme.apply();
    });
    button.set_callback(move |_| {
        choice.set_value(-1);
        reset_color_map();
    });

    a.run().unwrap();
}
