# fltk-theme

A theming crate for fltk-rs, based on work by [Remy Oukaour](https://github.com/roukaour/viz-brain-visualizer) and [Greg Ercolano](https://groups.google.com/g/fltkgeneral/c/3A5VC_854ok/m/sDpJsmuLBAAJ).

## Usage
```toml
[dependencies]
fltk = "1.1.4"
fltk-theme = "0.1"
```

## Example

Setting the color theme:

```rust
use fltk::{prelude::*, *};
use fltk_theme::{ColorTheme, color_themes};

fn main() {
    let a = app::App::default().with_scheme(app::Scheme::Gtk);
    let theme = ColorTheme::from_colormap(color_themes::BLACK_THEME);
    theme.apply();
    let mut win = window::Window::default().with_size(400, 300);
    let mut btn = button::Button::new(160, 200, 80, 40, "Hello");
    btn.set_color(btn.color().lighter());
    win.end();
    win.show();
    a.run().unwrap();
}
```

Setting the widget theme:

```rust
use fltk::{prelude::*, *};
use fltk_theme::{widget_themes, WidgetTheme, ThemeType};

fn main() {
    let a = app::App::default();
    let widget_theme = WidgetTheme::new(ThemeType::Aqua);
    widget_theme.apply();
    let mut win = window::Window::default().with_size(400, 300);
    let mut btn = button::Button::new(160, 200, 80, 30, "Hello");
    btn.set_frame(widget_themes::OS_DEFAULT_BUTTON_UP_BOX);
    win.end();
    win.show();
    a.run().unwrap();
}
```

## Widget themes

- Classic (old Windows theme)
![alt_test](screenshots/classic.jpg)

- Aero (Windows 7 theme)
![alt_test](screenshots/aero.jpg)

- Aqua (MacOS theme)
![alt_test](screenshots/aqua.jpg)

- Dark
![alt_test](screenshots/dark.jpg)

- High Contrast
![alt_test](screenshots/high_contrast.jpg)

- Blue
![alt_test](screenshots/blue.jpg)

- Metro (Windows 8 theme)
![alt_test](screenshots/metro.jpg)

- Greybird (Gnome xfce)
![alt_test](screenshots/greybird.jpg)


## Color themes

- Black theme
![alt_test](screenshots/black.jpg)

- Dark theme
![alt_test](screenshots/dark_color.JPG)

- Plain gray theme
![alt_test](screenshots/plain-gray.jpg)

- Tan theme
![alt_test](screenshots/tan.jpg)

- Shake theme
![alt_test](screenshots/shake.jpg)
