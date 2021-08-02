# fltk-theme

A theming crate for fltk-rs, based on work by [Remy Oukaour](https://github.com/roukaour/viz-brain-visualizer) and [Greg Ercolano](https://groups.google.com/g/fltkgeneral/c/3A5VC_854ok/m/sDpJsmuLBAAJ), and schemes developed by the NTK GUI library.

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
![alt_test](screenshots/dark_color.jpg)

- Plain gray theme
![alt_test](screenshots/plain-gray.jpg)

- Tan theme
![alt_test](screenshots/tan.jpg)

- Shake theme
![alt_test](screenshots/shake.jpg)

## Theme FrameTypes

Choosing a WidgetTheme will also define a set of FrameTypes which can be used for your widgets.
```
OS_BUTTON_UP_BOX
OS_CHECK_DOWN_BOX
OS_BUTTON_UP_FRAME
OS_CHECK_DOWN_FRAME
OS_PANEL_THIN_UP_BOX
OS_SPACER_THIN_DOWN_BOX
OS_PANEL_THIN_UP_FRAME
OS_SPACER_THIN_DOWN_FRAME
OS_RADIO_ROUND_DOWN_BOX
OS_HOVERED_UP_BOX
OS_DEPRESSED_DOWN_BOX
OS_HOVERED_UP_FRAME
OS_DEPRESSED_DOWN_FRAME
OS_INPUT_THIN_DOWN_BOX
OS_INPUT_THIN_DOWN_FRAME
OS_MINI_BUTTON_UP_BOX
OS_MINI_DEPRESSED_DOWN_BOX
OS_MINI_BUTTON_UP_FRAME
OS_MINI_DEPRESSED_DOWN_FRAME
OS_DEFAULT_BUTTON_UP_BOX
OS_DEFAULT_HOVERED_UP_BOX
OS_DEFAULT_DEPRESSED_DOWN_BOX
OS_TOOLBAR_BUTTON_HOVER_BOX
OS_TABS_BOX
OS_SWATCH_BOX
OS_SWATCH_FRAME
OS_BG_BOX
```

You can check the frames example to see all `FrameType`'s you can apply to you widgets.
![alt_test](screenshots/frames.jpg)

## Widget Schemes

These provide schemes for widgets without color theming. Currently there are 4 schemes:
- Clean: Taken from NTK's clear scheme.
![alt_test](screenshots/clean.jpg)

- Crystal: Taken from NTK's crystal scheme.
![alt_test](screenshots/crystal.jpg)

- Gleam: Taken from NTK's gleam scheme.
![alt_test](screenshots/gleam.jpg)

- SvgBased: This overrides FLTK's Base scheme round/rounded/oval FrameTypes which are drawn using scalable vector graphics.
![alt_test](screenshots/svgbased.jpg)

