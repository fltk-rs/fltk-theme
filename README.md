# fltk-theme

A theming crate for fltk-rs, based on work by [Greg Ercolano](https://groups.google.com/g/fltkgeneral/c/3A5VC_854ok/m/sDpJsmuLBAAJ).

## Usage
```toml
[dependencies]
fltk = "1"
fltk-theme = "0.1"
```

## Example
```rust
use fltk::{prelude::*, *};
use fltk_theme::ThemeMap;

fn main() {
    let a = app::App::default().with_scheme(app::Scheme::Gtk);
    let theme = ThemeMap::load("dark-theme.map").unwrap();
    theme.theme();
    let mut win = window::Window::default().with_size(400, 300);
    win.end();
    win.show();
    a.run().unwrap();
}
```