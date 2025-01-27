#![allow(non_upper_case_globals)]

use fltk::enums::Color;
use fltk::utils::oncelock::Lazy;

pub static backgroundColor2: Lazy<Color> = Lazy::new(|| Color::from_rgb(230, 230, 230));
pub static windowBackgroundColor: Lazy<Color> = Lazy::new(|| Color::from_rgb(232, 234, 235));
pub static labelColor: Lazy<Color> = Lazy::new(|| Color::from_rgb(49, 54, 61));
pub static controlBackgroundColor: Lazy<Color> = Lazy::new(|| Color::from_rgb(230, 230, 230));
pub static secondaryLabelColor: Lazy<Color> = Lazy::new(|| Color::from_rgb(140, 142, 146));
pub static tertiaryLabelColor: Lazy<Color> = Lazy::new(|| Color::from_rgba_tuple((140, 142, 146, 191)));
pub static quaternaryLabelColor: Lazy<Color> = Lazy::new(|| Color::from_rgba_tuple((140, 142, 146, 127)));
pub static textColor: Lazy<Color> = Lazy::new(|| Color::from_rgb(49, 54, 61));
pub static placeholderTextColor: Lazy<Color> = Lazy::new(|| Color::from_rgb(140, 142, 146));
pub static selectedTextColor: Lazy<Color> = Lazy::new(|| Color::from_rgb(254, 254, 254));
pub static textBackgroundColor: Lazy<Color> = Lazy::new(|| Color::from_rgb(235, 240, 245));
pub static selectedTextBackgroundColor: Lazy<Color> =
    Lazy::new(|| Color::from_rgba_tuple((197, 14, 210, 255)));
pub static keyboardFocusIndicatorColor: Lazy<Color> =
    Lazy::new(|| Color::from_rgba_tuple((197, 14, 210, 191)));
pub static unemphasizedSelectedTextColor: Lazy<Color> = Lazy::new(|| Color::from_rgb(254, 254, 254)); 
pub static unemphasizedSelectedTextBackgroundColor: Lazy<Color> = Lazy::new(|| Color::from_rgb(232, 234, 236));
pub static linkColor: Lazy<Color> = Lazy::new(|| Color::from_rgb(82, 148, 226));
pub static separatorColor: Lazy<Color> = Lazy::new(|| Color::from_rgba_tuple((0, 0, 0, 63)));
pub static selectedContentBackgroundColor: Lazy<Color> =
    Lazy::new(|| Color::from_rgba_tuple((197, 14, 210, 255)));
pub static unemphasizedSelectedContentBackgroundColor: Lazy<Color> = Lazy::new(|| Color::from_rgb(232, 234, 236));
pub static selectedMenuItemTextColor: Lazy<Color> = Lazy::new(|| Color::from_rgb(254, 254, 254)); 
pub static gridColor: Lazy<Color> = Lazy::new(|| Color::from_rgb(22, 25, 37));
pub static headerTextColor: Lazy<Color> = Lazy::new(|| Color::from_rgb(49, 54, 61));
pub static origControlAccentColor: Lazy<Color> =
    Lazy::new(|| Color::from_rgba_tuple((0, 232, 198, 255))); // Sweet's original green color for checkboxes
pub static controlAccentColor: Lazy<Color> =
    Lazy::new(|| Color::from_rgba_tuple((197, 14, 210, 255)));
pub static controlColor: Lazy<Color> = Lazy::new(|| Color::from_rgba_tuple((255, 255, 255, 255)));
pub static controlTextColor: Lazy<Color> = Lazy::new(|| Color::from_rgb(49, 54, 61));
pub static disabledControlTextColor: Lazy<Color> = Lazy::new(|| Color::from_rgba_tuple((140, 142, 146, 127)));
pub static selectedControlColor: Lazy<Color> = Lazy::new(|| Color::from_rgb(197, 14, 210));
pub static selectedControlTextColor: Lazy<Color> = Lazy::new(|| Color::from_rgb(49, 54, 61));
pub static alternateSelectedControlTextColor: Lazy<Color> = Lazy::new(|| Color::from_rgb(254, 254, 254));
pub static scrubberTexturedBackgroundColor: Lazy<Color> = Lazy::new(|| Color::from_rgb(211, 218, 227));
pub static windowFrameTextColor: Lazy<Color> = Lazy::new(|| Color::from_rgb(49, 54, 61));
pub static underPageBackgroundColor: Lazy<Color> = Lazy::new(|| Color::from_rgb(230, 230, 230));
pub static findHighlightColor: Lazy<Color> = Lazy::new(|| Color::from_rgb(255, 106, 0));
pub static highlightColor: Lazy<Color> = Lazy::new(|| Color::from_rgb(197, 14, 210));
pub static shadowColor: Lazy<Color> = Lazy::new(|| Color::from_rgba_tuple((0, 0, 0, 255)));
pub static systemBrownColor: Lazy<Color> = Lazy::new(|| Color::from_rgb(155, 123, 85));
pub static systemFuchsiaColor: Lazy<Color> = Lazy::new(|| Color::from_rgb(197, 14, 210));
pub static systemGrayColor: Lazy<Color> = Lazy::new(|| Color::from_rgb(133, 133, 139));
pub static systemGreenColor: Lazy<Color> = Lazy::new(|| Color::from_rgb(48, 211, 58));
pub static systemIndigoColor: Lazy<Color> = Lazy::new(|| Color::from_rgb(74, 64, 223));
pub static systemOrangeColor: Lazy<Color> = Lazy::new(|| Color::from_rgb(252, 141, 13));
pub static systemPinkColor: Lazy<Color> = Lazy::new(|| Color::from_rgb(251, 25, 76));
pub static systemPurpleColor: Lazy<Color> = Lazy::new(|| Color::from_rgb(157, 51, 213));
pub static systemRedColor: Lazy<Color> = Lazy::new(|| Color::from_rgb(251, 43, 44));
pub static systemTealColor: Lazy<Color> = Lazy::new(|| Color::from_rgba_tuple((76, 187, 242, 255)));
pub static systemYellowColor: Lazy<Color> = Lazy::new(|| Color::from_rgba_tuple((254, 207, 14, 255)));
pub static systemBlueColor: Lazy<Color> = Lazy::new(|| Color::from_rgba_tuple((16, 106, 254, 255)));
pub static systemCyanColor: Lazy<Color> = Lazy::new(|| Color::from_rgba_tuple((90, 200, 245, 255)));
