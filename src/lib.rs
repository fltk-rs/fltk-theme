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
use fltk_theme::Theme;

let a = app::App::default().with_scheme(app::Scheme::Gtk);
let theme = Theme::load("examples/themes/black.map").unwrap();
theme.apply();
let mut win = window::Window::default().with_size(400, 300);
let mut btn = button::Button::new(160, 200, 80, 40, "Hello");
btn.set_color(btn.color().lighter());
win.end();
win.show();
a.run().unwrap();
```
*/

use fltk::{app, enums::Color, prelude::FltkError};
use std::path;

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
pub struct Theme(Vec<ColorMap>);

impl Theme {
    fn load_(path: &path::Path) -> Result<Theme, FltkError> {
        let buf = std::fs::read_to_string(path)?;
        Ok(Self::from_string(&buf))
    }

    /// Load from a theme file
    pub fn load<P: AsRef<path::Path>>(path: P) -> Result<Theme, FltkError> {
        Self::load_(path.as_ref())
    }

    /// Load from a string
    /// # Panics
    /// Panics on parse failure
    pub fn from_string(buf: &str) -> Theme {
        let mut vec: Vec<ColorMap> = vec![];
        for line in buf.lines() {
            let line = line.trim_start();
            if line.starts_with("cmap") {
                let map: Vec<&str> = line.split_whitespace().collect();
                let cmap = ColorMap {
                    index: map[1].parse().expect("Parse Error!"),
                    r: map[2].parse().expect("Parse Error!"),
                    g: map[3].parse().expect("Parse Error!"),
                    b: map[4].parse().expect("Parse Error!"),
                };
                vec.push(cmap);
            }
        }
        Theme(vec)
    }

    /// Load from a color map
    pub fn from_colormap(map: &[ColorMap]) -> Theme {
        Theme(map.to_vec())
    }

    /// apply() the theme
    pub fn apply(&self) {
        for elem in &self.0 {
            app::set_color(Color::by_index(elem.index), elem.r, elem.g, elem.b);
        }
        app::reload_scheme().ok();
    }
}
