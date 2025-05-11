use crate::{cmap, ColorMap};
use fltk::utils::oncelock::Lazy;

pub const LIGHT: &[crate::ColorMap] = &[
    cmap!(0, 55, 55, 55),     // foreground
    cmap!(7, 255, 255, 255),  // background2
    cmap!(15, 0, 120, 180),   // selection
    cmap!(49, 235, 235, 235), // background
];

pub const DARK1: &[crate::ColorMap] = &[
    cmap!(49, 55, 55, 55),
    cmap!(7, 75, 75, 75),
    cmap!(0, 235, 235, 235),
    cmap!(15, 0, 120, 180),
];

pub const DARK2: &[crate::ColorMap] = &[
    cmap!(49, 35, 35, 40),
    cmap!(7, 26, 26, 30),
    cmap!(0, 235, 235, 235),
    cmap!(15, 0, 120, 180),
];

pub const TAN: &[crate::ColorMap] = &[
    cmap!(49, 195, 195, 181),
    cmap!(7, 243, 243, 243),
    cmap!(0, 55, 55, 55),
    cmap!(15, 0, 0, 175),
];

pub const DARK_TAN: &[crate::ColorMap] = &[
    cmap!(49, 165, 165, 151),
    cmap!(7, 223, 223, 223),
    cmap!(0, 55, 55, 55),
    cmap!(15, 0, 0, 175),
];

pub const MARINE: &[crate::ColorMap] = &[
    cmap!(49, 136, 192, 184),
    cmap!(7, 200, 224, 216),
    cmap!(0, 55, 55, 55),
    cmap!(15, 0, 0, 128),
];

pub const BLUEISH: &[crate::ColorMap] = &[
    cmap!(49, 210, 213, 215),
    cmap!(7, 255, 255, 255),
    cmap!(0, 55, 55, 55),
    cmap!(15, 0, 0, 128),
];

pub const NORD: &[crate::ColorMap] = &[
    cmap!(49, 41, 46, 57),
    cmap!(7, 59, 66, 82),
    cmap!(0, 235, 235, 235),
    cmap!(15, 0, 120, 180),
];

pub const HIGH_CONTRAST: &[crate::ColorMap] = &[
    cmap!(49, 0, 0, 0),
    cmap!(7, 20, 20, 20),
    cmap!(0, 255, 255, 255),
    cmap!(15, 0, 120, 255),
];

pub const FOREST: &[crate::ColorMap] = &[
    cmap!(49, 34, 51, 34),
    cmap!(7, 46, 79, 46),
    cmap!(0, 200, 200, 200),
    cmap!(15, 64, 224, 208),
];

pub const PURPLE_DUSK: &[crate::ColorMap] = &[
    cmap!(49, 48, 25, 52),
    cmap!(7, 72, 50, 72),
    cmap!(0, 220, 220, 220),
    cmap!(15, 255, 105, 180),
];

pub const SOLARIZED_LIGHT: &[crate::ColorMap] = &[
    cmap!(49, 253, 246, 227),
    cmap!(7, 238, 232, 213),
    cmap!(0, 101, 123, 131),
    cmap!(15, 38, 139, 210),
];

pub const SOLARIZED_DARK: &[crate::ColorMap] = &[
    cmap!(49, 0, 43, 54),
    cmap!(7, 7, 54, 66),
    cmap!(0, 131, 148, 150),
    cmap!(15, 211, 54, 130),
];

pub const MONOKAI: &[crate::ColorMap] = &[
    cmap!(49, 39, 40, 34),
    cmap!(7, 51, 52, 46),
    cmap!(0, 249, 249, 249),
    cmap!(15, 152, 159, 177),
];

pub const GRUVBOX_LIGHT: &[crate::ColorMap] = &[
    cmap!(49, 251, 237, 193),
    cmap!(7, 235, 219, 178),
    cmap!(0, 56, 52, 46),
    cmap!(15, 69, 133, 137),
];

pub const GRUVBOX_DARK: &[crate::ColorMap] = &[
    cmap!(49, 40, 40, 40),
    cmap!(7, 60, 60, 60),
    cmap!(0, 220, 208, 184),
    cmap!(15, 131, 165, 163),
];

pub const DRACULA: &[crate::ColorMap] = &[
    cmap!(49, 40, 42, 54),
    cmap!(7, 68, 71, 90),
    cmap!(0, 248, 248, 242),
    cmap!(15, 189, 147, 249),
];

pub const OCEANIC_NEXT: &[crate::ColorMap] = &[
    cmap!(49, 45, 52, 54),
    cmap!(7, 60, 68, 70),
    cmap!(0, 220, 220, 220),
    cmap!(15, 99, 184, 219),
];

pub const MINIMALIST: &[crate::ColorMap] = &[
    cmap!(49, 240, 240, 240),
    cmap!(7, 230, 230, 230),
    cmap!(0, 50, 50, 50),
    cmap!(15, 100, 149, 237),
];

pub const AUTUMN: &[crate::ColorMap] = &[
    cmap!(49, 245, 245, 220),
    cmap!(7, 230, 230, 200),
    cmap!(0, 80, 50, 10),
    cmap!(15, 255, 165, 0),
];

pub const CYBERPUNK: &[crate::ColorMap] = &[
    cmap!(49, 30, 30, 75),
    cmap!(7, 20, 20, 50),
    cmap!(0, 0, 255, 0),
    cmap!(15, 255, 0, 255),
];

pub const MATERIAL_DARK: &[crate::ColorMap] = &[
    cmap!(49, 28, 28, 28),
    cmap!(7, 40, 40, 40),
    cmap!(0, 255, 255, 255),
    cmap!(15, 0, 122, 255),
];

pub const MINT: &[crate::ColorMap] = &[
    cmap!(49, 200, 240, 200),
    cmap!(7, 180, 220, 180),
    cmap!(0, 50, 100, 50),
    cmap!(15, 0, 255, 0),
];

pub const VINTAGE: &[crate::ColorMap] = &[
    cmap!(49, 240, 230, 200),
    cmap!(7, 220, 210, 180),
    cmap!(0, 80, 50, 30),
    cmap!(15, 255, 215, 0),
];

pub const GRAY: &[crate::ColorMap] = &[
    cmap!(49, 192, 192, 192),
    cmap!(7, 255, 255, 255),
    cmap!(0, 0, 0, 0),
    cmap!(15, 0, 0, 128),
];
