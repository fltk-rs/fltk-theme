/*!
# fltk-theme

A theming crate for fltk-rs.
- The widget themes are based on work by [Remy Oukaour](https://github.com/roukaour/viz-brain-visualizer) and [Rangi42](https://github.com/Rangi42/tilemap-studio).
- The color themes are based on work by [Greg Ercolano](https://groups.google.com/g/fltkgeneral/c/3A5VC_854ok/m/sDpJsmuLBAAJ).
- The widget schemes are based on work by the NTK GUI library.

## Usage
```toml
[dependencies]
fltk = "1.1.6"
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

Setting the widget scheme:
```rust
use fltk::{prelude::*, *};
use fltk_theme::{WidgetScheme, SchemeType};

fn main() {
    let a = app::App::default();
    let widget_scheme = WidgetScheme::new(SchemeType::Clean);
    widget_scheme.apply();
    let mut win = window::Window::default().with_size(400, 300);
    let mut btn = button::Button::new(160, 200, 80, 30, "Hello");
    win.end();
    win.show();
    a.run().unwrap();
}
```
*/
#![allow(clippy::needless_doctest_main)]

use fltk::{app, enums::Color};
#[cfg(target_os = "macos")]
mod cocoa_helper;
pub mod color_themes;
pub mod widget_schemes;
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

pub(crate) fn activated_color(c: Color) -> Color {
    if fltk::app::draw_frame_active() {
        c
    } else {
        c.inactive()
    }
}

/// Lists supported themes
#[derive(Debug, Clone, Copy)]
pub enum ThemeType {
    /// Windows classic
    Classic,
    /// Windows 7
    Aero,
    /// Windows 8
    Metro,
    /// Classic MacOS
    AquaClassic,
    /// Modern MacOS using system colors, Dark theme
    AquaDark,
    /// Modern MacOS using system colors, light theme
    AquaLight,
    /// Xfce
    Greybird,
    /// Windows 2000
    Blue,
    /// Dark
    Dark,
    /// High Contrast
    HighContrast,
    /// Windows 10
    Fluent,
}

#[derive(Debug, Clone, Copy)]
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
            ThemeType::AquaClassic => widget_themes::aqua_classic::use_aqua_classic_theme(),
            ThemeType::AquaDark => widget_themes::aqua_dark::use_aqua_dark_theme(),
            ThemeType::AquaLight => widget_themes::aqua_light::use_aqua_light_theme(),
            ThemeType::Dark => widget_themes::dark::use_dark_theme(),
            ThemeType::HighContrast => widget_themes::high_contrast::use_high_contrast_theme(),
            ThemeType::Blue => widget_themes::blue::use_blue_theme(),
            ThemeType::Metro => widget_themes::metro::use_metro_theme(),
            ThemeType::Greybird => widget_themes::greybird::use_greybird_theme(),
            ThemeType::Fluent => widget_themes::fluent::use_fluent_theme(),
        }
    }
}

/// Lists supported schemes
#[derive(Debug, Clone, Copy)]
pub enum SchemeType {
    /// Taken from the NTK fork
    Clean,
    /// Taken from the NTK fork
    Crystal,
    /// Taken from the NTK fork, a modification of the FLTK Gleam scheme
    Gleam,
    /**
    Draws the following FrameTypes using scalable vector graphics:
    - RoundedFrame
    - RoundedBox
    - RFlatBox
    - OvalBox
    - OvalFrame
    - OFlatFrame
    */
    SvgBased,
}

#[derive(Debug, Clone, Copy)]
pub struct WidgetScheme {
    scheme: SchemeType,
}

impl WidgetScheme {
    /// Create a Widget theme object
    pub fn new(scheme: SchemeType) -> Self {
        Self { scheme }
    }

    /// Apply the widget theme
    pub fn apply(&self) {
        match self.scheme {
            SchemeType::Clean => widget_schemes::clean::use_clean_scheme(),
            SchemeType::Crystal => widget_schemes::crystal::use_crystal_scheme(),
            SchemeType::Gleam => widget_schemes::gleam::use_gleam_scheme(),
            SchemeType::SvgBased => widget_schemes::svg_based::use_svg_based_scheme(),
        }
    }
}
