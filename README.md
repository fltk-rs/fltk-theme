# fltk-theme

A theming crate for fltk-rs.
- The widget themes are based on work by [Remy Oukaour](https://github.com/roukaour/viz-brain-visualizer) and [Rangi42](https://github.com/Rangi42/tilemap-studio).
- The color themes are based on work by [Greg Ercolano](https://groups.google.com/g/fltkgeneral/c/3A5VC_854ok/m/sDpJsmuLBAAJ).
- The fleet color themes are based on [CyprinusCarpio FLEET library](https://github.com/CyprinusCarpio/fleet).
- Some of the widget schemes are based on work by the NTK GUI library, the fleet schemes are based on the [FLEET library](https://github.com/CyprinusCarpio/fleet). Others are nouveau.

## Definitions

### Color theme
- A color theme modify FLTK's color map without modifying any of the drawing routines of the widgets.

### Widget scheme
- A widget scheme modifies FLTK's drawing routines for widgets, thus changing their appearance, without changing their colors.

### Widget theme
- A widget theme modifies FLTK's drawing routines for widgets, as well as the color map, or default colors used for drawing widgets.

As such, color themes can be combined with widget schemes. Widget schemes also enable individually changing the colors and selection colors of each widget. While widget themes have a fixed coloring for widgets, and these usually follow the theme they're named after.

## Usage
```toml
[dependencies]
fltk = "1.4"
fltk-theme = "0.7"
```

## Example

Setting the color theme:

```rust,no_run
use fltk::{prelude::*, *};
use fltk_theme::{ColorTheme, color_themes};

fn main() {
    let a = app::App::default().with_scheme(app::Scheme::Gtk);
    let theme = ColorTheme::new(color_themes::BLACK_THEME);
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

```rust,no_run
use fltk::{prelude::*, *};
use fltk_theme::{widget_themes, WidgetTheme, ThemeType};

fn main() {
    let a = app::App::default();
    let widget_theme = WidgetTheme::new(ThemeType::AquaClassic);
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
```rust,no_run
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


## Color themes

- Black theme
- ![Image](https://github.com/user-attachments/assets/23da332d-c3e3-4e91-beff-bcb8f9bc3cb5)

- Dark theme
- ![Image](https://github.com/user-attachments/assets/c7bf424c-1aa9-4d5f-b039-1bbe75c7dd6a)

- Plain gray theme
- ![Image](https://github.com/user-attachments/assets/5a928515-8c30-400e-97aa-5c254949d09b)

- Tan theme
- ![Image](https://github.com/user-attachments/assets/d3951a56-a75e-4f1b-9c6c-19d552f8108e)

- Shake theme
- ![Image](https://github.com/user-attachments/assets/ffaa38c2-69f6-4dca-83eb-2a1e79063b05)

- Fleet themes (From the excellent [FLEET library](https://github.com/CyprinusCarpio/fleet)):
    - GRUVBOX_DARK
    - ![Image](https://github.com/user-attachments/assets/e60d11fc-cd78-4963-a143-ccf7f6fdea60)

    - DRACULA
    - ![Image](https://github.com/user-attachments/assets/7727fdc7-407e-43b4-ab87-0f6b7a57d17a)

    - PURPLE_DUSK
    - ![Image](https://github.com/user-attachments/assets/9a7666a5-31b1-47cb-8975-99c3d8d7019c)

    - MONOKAI
    - ![Image](https://github.com/user-attachments/assets/50e9ee50-a526-45af-b9b4-ac7f7207fd5d)

    - CYBERPUNK
    - ![Image](https://github.com/user-attachments/assets/c54c7ff5-c7c5-43cb-bad6-71ed3b77128c)

    - SOLARIZED_DARK
    - ![Image](https://github.com/user-attachments/assets/c714b848-6e4b-4470-b31b-b9e4bc7cf24f)

    - MATERIAL_DARK
    - ![Image](https://github.com/user-attachments/assets/8f1aa700-14fe-4f06-921a-83a2ef6e9e15)

    - And others.

## Widget themes

- Classic (old Windows theme)
- ![Image](https://github.com/user-attachments/assets/6f259e62-0f1e-49e6-8611-dbb91f86c219)

- Aero (Windows 7 theme)
- ![Image](https://github.com/user-attachments/assets/4dd10bde-91f0-4c83-a5e6-95eb26182a4d)

- AquaClassic (classic MacOS theme),
- ![Image](https://github.com/user-attachments/assets/846ab758-e6e5-4389-9bc8-111f64864248)

- Dark
- ![Image](https://github.com/user-attachments/assets/9094dfe7-4fd5-4957-9888-8476e8266df4)

- High Contrast
- ![Image](https://github.com/user-attachments/assets/e0b0adfc-6d93-4c1c-863c-59c6352ded24)

- Blue
- ![Image](https://github.com/user-attachments/assets/4aa320b6-38d8-4d99-9d87-f91b8492af16)

- Metro (Windows 8 theme)
- ![Image](https://github.com/user-attachments/assets/f545e8bb-8317-43ce-b05c-6ce23bf6a956)

- Greybird (Gnome xfce)
- ![Image](https://github.com/user-attachments/assets/98778e37-24e6-4b02-bac7-b478854d99d7)

## Theme FrameTypes

Choosing a WidgetTheme will also define a set of FrameTypes which can be used for your widgets.
```ignore
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

## Widget Schemes

These provide schemes for widgets without color theming. Currently there are 7 schemes:
- Clean: Taken from NTK's clear scheme.
- ![Image](https://github.com/user-attachments/assets/2c0bc8da-2d8c-4283-8bbf-2148a2cdc6a2)

- Aqua: Tries to mimic the modern MacOS's styles.
- ![alt_test](screenshots/aqua_scheme.jpg)
- ![alt_test](screenshots/aqua_scheme2.jpg)

- Fluent: Tries to mimic Window's 10 styles.
- ![alt_test](screenshots/fluent.jpg)
- ![alt_test](screenshots/fluent_dark.jpg)

- SvgBased: This overrides FLTK's Base scheme round/rounded/oval FrameTypes which are drawn using scalable vector graphics.
- ![alt_test](screenshots/svgbased.jpg)

- Sweet: Tries to mimic the Sweet theme for GNOME/KDE.
- ![alt_test](screenshots/sweet_dark.png)
- ![alt_test](screenshots/sweet_light.png)

- Fleet1: A 3D scheme designed for good looks in both dark and light colors.
- Fleet2: A gradient scheme designed for good looks in both dark and light colors.

## Colors

The crate also provides colors, namely html colors and aqua colors.
The aqua colors are provided as static values and are named after the cocoa NSColor properties (such as windowBackgroundColor, systemBlueColor, controlAccentColor...etc). The html colors are provided in a static HashMap and can be accessed by their [html names](https://www.w3schools.com/tags/ref_colornames.asp). Refer to the html_colors and aqua_dark examples to see how the colors are used.

Colors and Color themes can also be used with widget schemes or even in a regular fltk-rs application.
