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
```rust,no_run
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
pub enum WidgetThemeType {
    Classic,
    Aero,
    Metro,
    Aqua,
    Greybird,
    Ocean,
    Blue,
    Olive,
    RoseGold,
    Dark,
    BrushedMetal,
    HighContrast,
}

pub struct WidgetTheme {
    theme: WidgetThemeType,
}

impl WidgetTheme {
    /// Create a Widget theme object
    pub fn new(theme: WidgetThemeType) -> Self {
        Self {
            theme
        }
    }

    /// Apply the widget theme
    pub fn apply(&self) {
        match self.theme {
            WidgetThemeType::Classic => widget_themes::classic::use_classic_theme(),
            WidgetThemeType::Aero => widget_themes::aero::use_aero_theme(),
            _ => (),
        }
    }
}