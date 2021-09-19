#[cfg(target_os = "macos")]
fn main() {
    use fltk::{prelude::*, *};
    use fltk_theme::{widget_themes, WidgetTheme, ThemeType};

    let a = app::App::default();
    let widget_theme = WidgetTheme::new(ThemeType::Aqua2);
    widget_theme.apply();
    let mut win = window::Window::default().with_size(400, 300);
    let mut btn = button::Button::new(160, 200, 80, 30, "Hello");
    btn.set_frame(widget_themes::OS_DEFAULT_BUTTON_UP_BOX);
    win.end();
    win.show();
    a.run().unwrap();
}

#[cfg(not(target_os = "macos"))]
fn main() {}