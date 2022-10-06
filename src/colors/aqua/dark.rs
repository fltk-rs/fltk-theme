#![allow(non_upper_case_globals)]

use fltk::enums::Color;

lazy_static::lazy_static! {
    pub static ref backgroundColor2: Color = Color::from_rgba_tuple((0, 0, 0, 255));
    pub static ref windowBackgroundColor: Color = Color::from_rgba_tuple((37, 37, 37, 255));
    pub static ref labelColor: Color = Color::from_rgba_tuple((255, 254, 254, 216));
    pub static ref controlBackgroundColor: Color = Color::from_rgba_tuple((22, 22, 22, 255));
    pub static ref secondaryLabelColor: Color = Color::from_rgba_tuple((255, 254, 254, 140));
    pub static ref tertiaryLabelColor: Color = Color::from_rgba_tuple((255, 254, 254, 63));
    pub static ref quaternaryLabelColor: Color = Color::from_rgba_tuple((255, 254, 254, 25));
    pub static ref textColor: Color = Color::from_rgba_tuple((255, 254, 254, 255));
    pub static ref placeholderTextColor: Color = Color::from_rgba_tuple((255, 254, 254, 63));
    pub static ref selectedTextColor: Color = Color::from_rgba_tuple((255, 255, 255, 255));
    pub static ref textBackgroundColor: Color = Color::from_rgba_tuple((22, 22, 22, 255));
    pub static ref selectedTextBackgroundColor: Color = Color::from_rgba_tuple((48, 79, 120, 255));
    pub static ref keyboardFocusIndicatorColor: Color = Color::from_rgba_tuple((27, 149, 254, 76));
    pub static ref unemphasizedSelectedTextColor: Color = Color::from_rgba_tuple((255, 255, 255, 255));
    pub static ref unemphasizedSelectedTextBackgroundColor: Color = Color::from_rgba_tuple((54, 54, 54, 255));
    pub static ref linkColor: Color = Color::from_rgba_tuple((52, 134, 254, 255));
    pub static ref separatorColor: Color = Color::from_rgba_tuple((255, 254, 254, 25));
    pub static ref selectedContentBackgroundColor: Color = Color::from_rgba_tuple((5, 63, 197, 255));
    pub static ref unemphasizedSelectedContentBackgroundColor: Color = Color::from_rgba_tuple((54, 54, 54, 255));
    pub static ref selectedMenuItemTextColor: Color = Color::from_rgba_tuple((255, 254, 254, 255));
    pub static ref gridColor: Color = Color::from_rgba_tuple((20, 20, 20, 255));
    pub static ref headerTextColor: Color = Color::from_rgba_tuple((255, 254, 254, 255));
    pub static ref controlAccentColor: Color = Color::from_rgba_tuple((10, 95, 254, 255));
    pub static ref controlColor: Color = Color::from_rgba_tuple((255, 254, 254, 63));
    pub static ref controlTextColor: Color = Color::from_rgba_tuple((255, 254, 254, 216));
    pub static ref disabledControlTextColor: Color = Color::from_rgba_tuple((255, 254, 254, 63));
    pub static ref selectedControlColor: Color = Color::from_rgba_tuple((48, 79, 120, 255));
    pub static ref selectedControlTextColor: Color = Color::from_rgba_tuple((255, 254, 254, 216));
    pub static ref alternateSelectedControlTextColor: Color = Color::from_rgba_tuple((255, 254, 254, 255));
    pub static ref scrubberTexturedBackgroundColor: Color = Color::from_rgba_tuple((255, 254, 254, 255));
    pub static ref windowFrameTextColor: Color = Color::from_rgba_tuple((255, 254, 254, 216));
    pub static ref underPageBackgroundColor: Color = Color::from_rgba_tuple((29, 29, 29, 255));
    pub static ref findHighlightColor: Color = Color::from_rgba_tuple((255, 255, 10, 255));
    pub static ref highlightColor: Color = Color::from_rgba_tuple((164, 164, 164, 255));
    pub static ref shadowColor: Color = Color::from_rgba_tuple((0, 0, 0, 255));
    pub static ref systemBrownColor: Color = Color::from_rgba_tuple((155, 123, 85, 255));
    pub static ref systemGrayColor: Color = Color::from_rgba_tuple((133, 133, 139, 255));
    pub static ref systemGreenColor: Color = Color::from_rgba_tuple((48, 211, 58, 255));
    pub static ref systemIndigoColor: Color = Color::from_rgba_tuple((74, 64, 223, 255));
    pub static ref systemOrangeColor: Color = Color::from_rgba_tuple((252, 141, 13, 255));
    pub static ref systemPinkColor: Color = Color::from_rgba_tuple((251, 25, 76, 255));
    pub static ref systemPurpleColor: Color = Color::from_rgba_tuple((175, 56, 238, 255));
    pub static ref systemRedColor: Color = Color::from_rgba_tuple((251, 43, 44, 255));
    pub static ref systemTealColor: Color = Color::from_rgba_tuple((76, 187, 242, 255));
    pub static ref systemYellowColor: Color = Color::from_rgba_tuple((254, 207, 14, 255));
    pub static ref systemBlueColor: Color = Color::from_rgba_tuple((16, 106, 254, 255));
    pub static ref systemCyanColor: Color = Color::from_rgba_tuple((90, 200, 245, 255));
}
