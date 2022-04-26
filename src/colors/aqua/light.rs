#![allow(non_upper_case_globals)]

use fltk::enums::Color;

lazy_static::lazy_static! {
    pub static ref backgroundColor2: Color = Color::from_rgba_tuple((255, 255, 255, 255));
    pub static ref windowBackgroundColor: Color = Color::from_rgba_tuple((231, 231, 231, 255));
    pub static ref labelColor: Color = Color::from_rgba_tuple((0, 0, 0, 216));
    pub static ref controlBackgroundColor: Color = Color::from_rgba_tuple((255, 254, 254, 255));
    pub static ref secondaryLabelColor: Color = Color::from_rgba_tuple((0, 0, 0, 127));
    pub static ref tertiaryLabelColor: Color = Color::from_rgba_tuple((0, 0, 0, 66));
    pub static ref quaternaryLabelColor: Color = Color::from_rgba_tuple((0, 0, 0, 25));
    pub static ref textColor: Color = Color::from_rgba_tuple((0, 0, 0, 255));
    pub static ref placeholderTextColor: Color = Color::from_rgba_tuple((0, 0, 0, 63));
    pub static ref selectedTextColor: Color = Color::from_rgba_tuple((0, 0, 0, 255));
    pub static ref textBackgroundColor: Color = Color::from_rgba_tuple((255, 254, 254, 255));
    pub static ref selectedTextBackgroundColor: Color = Color::from_rgba_tuple((164, 204, 254, 255));
    pub static ref keyboardFocusIndicatorColor: Color = Color::from_rgba_tuple((7, 75, 240, 63));
    pub static ref unemphasizedSelectedTextColor: Color = Color::from_rgba_tuple((0, 0, 0, 255));
    pub static ref unemphasizedSelectedTextBackgroundColor: Color = Color::from_rgba_tuple((211, 211, 211, 255));
    pub static ref linkColor: Color = Color::from_rgba_tuple((8, 79, 209, 255));
    pub static ref separatorColor: Color = Color::from_rgba_tuple((0, 0, 0, 25));
    pub static ref selectedContentBackgroundColor: Color = Color::from_rgba_tuple((7, 73, 217, 255));
    pub static ref unemphasizedSelectedContentBackgroundColor: Color = Color::from_rgba_tuple((211, 211, 211, 255));
    pub static ref selectedMenuItemTextColor: Color = Color::from_rgba_tuple((255, 254, 254, 255));
    pub static ref gridColor: Color = Color::from_rgba_tuple((223, 223, 223, 255));
    pub static ref headerTextColor: Color = Color::from_rgba_tuple((0, 0, 0, 216));
    pub static ref controlAccentColor: Color = Color::from_rgba_tuple((10, 95, 254, 255));
    pub static ref controlColor: Color = Color::from_rgba_tuple((255, 254, 254, 255));
    pub static ref controlTextColor: Color = Color::from_rgba_tuple((0, 0, 0, 216));
    pub static ref disabledControlTextColor: Color = Color::from_rgba_tuple((0, 0, 0, 63));
    pub static ref selectedControlColor: Color = Color::from_rgba_tuple((164, 204, 254, 255));
    pub static ref selectedControlTextColor: Color = Color::from_rgba_tuple((0, 0, 0, 216));
    pub static ref alternateSelectedControlTextColor: Color = Color::from_rgba_tuple((255, 254, 254, 255));
    pub static ref scrubberTexturedBackgroundColor: Color = Color::from_rgba_tuple((255, 254, 254, 255));
    pub static ref windowFrameTextColor: Color = Color::from_rgba_tuple((0, 0, 0, 216));
    pub static ref underPageBackgroundColor: Color = Color::from_rgba_tuple((131, 131, 131, 229));
    pub static ref findHighlightColor: Color = Color::from_rgba_tuple((255, 255, 10, 255));
    pub static ref highlightColor: Color = Color::from_rgba_tuple((255, 254, 254, 255));
    pub static ref shadowColor: Color = Color::from_rgba_tuple((0, 0, 0, 255));
    pub static ref systemBrownColor: Color = Color::from_rgba_tuple((144, 113, 75, 255));
    pub static ref systemGrayColor: Color = Color::from_rgba_tuple((123, 123, 128, 255));
    pub static ref systemGreenColor: Color = Color::from_rgba_tuple((40, 199, 50, 255));
    pub static ref systemIndigoColor: Color = Color::from_rgba_tuple((69, 59, 204, 255));
    pub static ref systemOrangeColor: Color = Color::from_rgba_tuple((252, 129, 8, 255));
    pub static ref systemPinkColor: Color = Color::from_rgba_tuple((251, 12, 67, 255));
    pub static ref systemPurpleColor: Color = Color::from_rgba_tuple((157, 51, 213, 255));
    pub static ref systemRedColor: Color = Color::from_rgba_tuple((251, 32, 37, 255));
    pub static ref systemTealColor: Color = Color::from_rgba_tuple((71, 175, 235, 255));
    pub static ref systemYellowColor: Color = Color::from_rgba_tuple((253, 194, 9, 255));
    pub static ref systemBlueColor: Color = Color::from_rgba_tuple((10, 95, 254, 255));
    pub static ref systemCyanColor: Color = Color::from_rgba_tuple((85, 190, 240, 255));
}

