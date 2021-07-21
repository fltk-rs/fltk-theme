/*!
# fltk-theme

A theming crate for fltk-rs, based on work by [Greg Ercolano](https://groups.google.com/g/fltkgeneral/c/3A5VC_854ok/m/sDpJsmuLBAAJ).

## Usage
```toml
[dependencies]
fltk = "1"
fltk-theme = "0.1"
```
*/

use fltk::{app, enums::Color, prelude::FltkError};
use std::path;

/// Color map struct
#[derive(Default, Clone, Debug)]
pub struct ColorMap {
    index: u8,
    r: u8,
    g: u8,
    b: u8,
}

/// A theme is just a Vec of colormaps
#[derive(Debug, Clone)]
pub struct Theme(Vec<ColorMap>);

/// Predefined themes
pub enum BuiltinTheme {
    Dark,
    Black,
    PlainGray,
    Shake,
    Tan,
}

impl Theme {
    fn load_(path: &path::Path) -> Result<Theme, FltkError> {
        let buf = std::fs::read_to_string(path)?;
        let mut vec: Vec<ColorMap> = vec![];
        for line in buf.lines() {
            let line = line.trim_start();
            if line.starts_with("cmap") {
                let map: Vec<&str> = line.split_whitespace().collect();
                let mut cmap = ColorMap::default();
                cmap.index = map[1].parse().expect("Parse Error!");
                cmap.r = map[2].parse().expect("Parse Error!");
                cmap.g = map[3].parse().expect("Parse Error!");
                cmap.b = map[4].parse().expect("Parse Error!");
                vec.push(cmap);
            }
        }
        Ok(Theme(vec))
    }

    /// Load from a theme file
    pub fn load<P: AsRef<path::Path>>(path: P) -> Result<Theme, FltkError> {
        Self::load_(path.as_ref())
    }

    /// Select the theme
    pub fn select(&self) {
        for elem in &self.0 {
            app::set_color(Color::by_index(elem.index), elem.r, elem.g, elem.b);
        }
        app::reload_scheme().ok();
    }
}



