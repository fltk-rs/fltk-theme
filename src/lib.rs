/*!
# fltk-theme

A theming crate for fltk-rs, based on work by [Greg Ercolano](https://groups.google.com/g/fltkgeneral/c/3A5VC_854ok/m/sDpJsmuLBAAJ).

## Usage
```toml
[dependencies]
fltk = "1.1.3"
fltk-theme = "0.1"
```

## Example

Setting the color theme:

```rust
use fltk::{prelude::*, *};
use fltk_theme::{ColorTheme, color_themes};

let a = app::App::default().with_scheme(app::Scheme::Gtk);
let theme = ColorTheme::from_colormap(color_themes::BLACK_THEME);
theme.apply();
let mut win = window::Window::default().with_size(400, 300);
let mut btn = button::Button::new(160, 200, 80, 40, "Hello");
btn.set_color(btn.color().lighter());
win.end();
win.show();
a.run().unwrap();
```

Setting the widget theme:

```rust
use fltk::{prelude::*, *};
use fltk_theme::{widget_themes, WidgetTheme, ThemeType};

let a = app::App::default();
let widget_theme = WidgetTheme::new(ThemeType::Aqua);
widget_theme.apply();
let mut win = window::Window::default().with_size(400, 300);
let mut btn = button::Button::new(160, 200, 80, 30, "Hello");
btn.set_frame(widget_themes::OS_DEFAULT_BUTTON_UP_BOX);
win.end();
win.show();
a.run().unwrap();
```
*/

use fltk::{app, enums::Color};
pub mod color_themes;
pub mod widget_themes;

/// Color map struct. (index, r, g, b)
#[derive(Default, Clone, Debug)]
pub struct ColorMap {
    pub index: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[macro_export]
macro_rules! cmap {
    ($i:tt, $r:tt, $g:tt, $b:tt) => {
        ColorMap {
            index: $i,
            r: $r,
            g: $g,
            b: $b,
        }
    };
}

/// A theme is just a Vec of colormaps
#[derive(Debug, Clone)]
pub struct ColorTheme(pub Vec<ColorMap>);

impl ColorTheme {
    /// Load from a color map
    pub fn from_colormap(map: &[ColorMap]) -> ColorTheme {
        ColorTheme(map.to_vec())
    }

    /// apply() the theme
    pub fn apply(&self) {
        for elem in &self.0 {
            app::set_color(Color::by_index(elem.index), elem.r, elem.g, elem.b);
        }
        app::reload_scheme().ok();
    }
}

/// Lists support themes
#[derive(Debug, Clone, Copy)]
pub enum ThemeType {
    Classic,
    Aero,
    Metro,
    Aqua,
    Greybird,
    Blue,
    Dark,
    HighContrast,
}

pub struct WidgetTheme {
    theme: ThemeType,
}

impl WidgetTheme {
    /// Create a Widget theme object
    pub fn new(theme: ThemeType) -> Self {
        Self { theme }
    }

    /// Apply the widget theme
    pub fn apply(&self) {
        match self.theme {
            ThemeType::Classic => widget_themes::classic::use_classic_theme(),
            ThemeType::Aero => widget_themes::aero::use_aero_theme(),
            ThemeType::Aqua => widget_themes::aqua::use_aqua_theme(),
            ThemeType::Dark => widget_themes::dark::use_dark_theme(),
            ThemeType::HighContrast => widget_themes::high_contrast::use_high_contrast_theme(),
            ThemeType::Blue => widget_themes::blue::use_blue_theme(),
            ThemeType::Metro => widget_themes::metro::use_metro_theme(),
            ThemeType::Greybird => widget_themes::greybird::use_greybird_theme(),
        }
    }
}
