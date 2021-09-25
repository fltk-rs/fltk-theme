use super::*;
#[cfg(target_os = "macos")]
use crate::cocoa_helper::*;
use fltk::{app, enums::Color, image, misc::Tooltip, prelude::ImageExt};

#[cfg(target_os = "macos")]
fn convert_colors(colors: (f64, f64, f64, f64)) -> (u8, u8, u8, u8) {
    let r = (colors.0 * 255.0) as u8;
    let g = (colors.1 * 255.0) as u8;
    let b = (colors.2 * 255.0) as u8;
    let a = (colors.3 * 255.0) as u8;
    (r, g, b, a)
}

#[cfg(target_os = "macos")]
macro_rules! get_colors {
    ($s:ident) => {{
        let mut r = 1.0;
        let mut g = 1.0;
        let mut b = 1.0;
        let mut a = 1.0;
        unsafe {
            $s(&mut r, &mut g, &mut b, &mut a);
        }
        convert_colors((r, g, b, a))
    }};
}

#[cfg(target_os = "macos")]
lazy_static::lazy_static! {
    static ref BG2_COL: (u8, u8, u8, u8) = (0, 0, 0, 255);
    static ref BG_COL: (u8, u8, u8, u8) = get_colors!(my_windowBackgroundColor);
    static ref FG_COL: (u8, u8, u8, u8) = get_colors!(my_labelColor);
    static ref CONTROL_COL: (u8, u8, u8, u8) = get_colors!(my_controlBackgroundColor);
    static ref FRAME_COL: (u8, u8, u8, u8) = get_colors!(my_windowFrameColor);
    static ref LABEL2_COL: (u8, u8, u8, u8) = get_colors!(my_secondaryLabelColor);
    static ref LABEL3_COL: (u8, u8, u8, u8) = get_colors!(my_tertiaryLabelColor);
    static ref LABEL4_COL: (u8, u8, u8, u8) = get_colors!(my_quaternaryLabelColor);
    static ref TXT_COL: (u8, u8, u8, u8) = get_colors!(my_textColor);
    static ref PH_TXT_COL: (u8, u8, u8, u8) = get_colors!(my_placeholderTextColor);
    static ref SEL_TXT_COL: (u8, u8, u8, u8) = get_colors!(my_selectedTextColor);
    static ref TXT_BG_COL: (u8, u8, u8, u8) = get_colors!(my_textBackgroundColor);
    static ref SEL_TXT_BG_COL: (u8, u8, u8, u8) = get_colors!(my_selectedTextBackgroundColor);
    static ref KB_IND_COL: (u8, u8, u8, u8) = get_colors!(my_keyboardFocusIndicatorColor);
    static ref SEL_TXT2_COL: (u8, u8, u8, u8) = get_colors!(my_unemphasizedSelectedTextColor);
    static ref SEL_TXT_BG2_COL: (u8, u8, u8, u8) = get_colors!(my_unemphasizedSelectedTextBackgroundColor);
    static ref LINK_COL: (u8, u8, u8, u8) = get_colors!(my_linkColor);
    static ref SEP_COL: (u8, u8, u8, u8) = get_colors!(my_separatorColor);
    static ref SEL_BG_COL: (u8, u8, u8, u8) = get_colors!(my_selectedContentBackgroundColor);
    static ref SEL_BG2_COL: (u8, u8, u8, u8) = get_colors!(my_unemphasizedSelectedContentBackgroundColor);
    static ref SEL_MEN_TXT_COL: (u8, u8, u8, u8) = get_colors!(my_selectedMenuItemTextColor);
    static ref GRID_COL: (u8, u8, u8, u8) = get_colors!(my_gridColor);
    static ref HDR_COL: (u8, u8, u8, u8) = get_colors!(my_headerTextColor);
    static ref CTRL_ACC_COL: (u8, u8, u8, u8) = get_colors!(my_controlAccentColor);
    static ref CTRL_COL: (u8, u8, u8, u8) = get_colors!(my_controlColor);
    static ref CTRL_TXT_COL: (u8, u8, u8, u8) = get_colors!(my_controlTextColor);
    static ref DIS_CTRL_TXT_COL: (u8, u8, u8, u8) = get_colors!(my_disabledControlTextColor);
    static ref SEL_CTRL_COL: (u8, u8, u8, u8) = get_colors!(my_selectedControlColor);
    static ref SEL_CTRL_TXT_COL: (u8, u8, u8, u8) = get_colors!(my_selectedControlTextColor);
    static ref ALT_SEL_CTRL_TXT_COL: (u8, u8, u8, u8) = get_colors!(my_alternateSelectedControlTextColor);
    static ref SCRUB_BG_COL: (u8, u8, u8, u8) = get_colors!(my_scrubberTexturedBackgroundColor);
    static ref WIN_FRM_TXT_COL: (u8, u8, u8, u8) = get_colors!(my_windowFrameTextColor);
    static ref PAGE_BG_COL: (u8, u8, u8, u8) = get_colors!(my_underPageBackgroundColor);
    static ref FIND_HLT_COL: (u8, u8, u8, u8) = get_colors!(my_findHighlightColor);
    static ref HLT_COL: (u8, u8, u8, u8) = get_colors!(my_highlightColor);
    static ref SHDW_COL: (u8, u8, u8, u8) = get_colors!(my_shadowColor);
}

#[cfg(not(target_os = "macos"))]
lazy_static::lazy_static! {
    static ref BG2_COL: (u8, u8, u8, u8) = (0, 0, 0, 255);
    static ref BG_COL: (u8, u8, u8, u8) = (37, 37, 37, 255);
    static ref FG_COL: (u8, u8, u8, u8) = (255, 254, 254, 216);
    static ref CONTROL_COL: (u8, u8, u8, u8) = (22, 22, 22, 255);
    static ref FRAME_COL: (u8, u8, u8, u8) = (153, 153, 153, 255);
    static ref LABEL2_COL: (u8, u8, u8, u8) = (255, 254, 254, 140);
    static ref LABEL3_COL: (u8, u8, u8, u8) = (255, 254, 254, 63);
    static ref LABEL4_COL: (u8, u8, u8, u8) = (255, 254, 254, 25);
    static ref TXT_COL: (u8, u8, u8, u8) = (255, 254, 254, 255);
    static ref PH_TXT_COL: (u8, u8, u8, u8) = (255, 254, 254, 63);
    static ref SEL_TXT_COL: (u8, u8, u8, u8) = (255, 255, 255, 255);
    static ref TXT_BG_COL: (u8, u8, u8, u8) = (22, 22, 22, 255);
    static ref SEL_TXT_BG_COL: (u8, u8, u8, u8) = (48, 79, 120, 255);
    static ref KB_IND_COL: (u8, u8, u8, u8) = (27, 149, 254, 76);
    static ref SEL_TXT2_COL: (u8, u8, u8, u8) = (255, 255, 255, 255);
    static ref SEL_TXT_BG2_COL: (u8, u8, u8, u8) = (54, 54, 54, 255);
    static ref LINK_COL: (u8, u8, u8, u8) = (52, 134, 254, 255);
    static ref SEP_COL: (u8, u8, u8, u8) = (255, 254, 254, 25);
    static ref SEL_BG_COL: (u8, u8, u8, u8) = (5, 63, 197, 255);
    static ref SEL_BG2_COL: (u8, u8, u8, u8) = (54, 54, 54, 255);
    static ref SEL_MEN_TXT_COL: (u8, u8, u8, u8) = (255, 254, 254, 255);
    static ref GRID_COL: (u8, u8, u8, u8) = (20, 20, 20, 255);
    static ref HDR_COL: (u8, u8, u8, u8) = (255, 254, 254, 255);
    static ref CTRL_ACC_COL: (u8, u8, u8, u8) = (10, 95, 254, 255);
    static ref CTRL_COL: (u8, u8, u8, u8) = (255, 254, 254, 63);
    static ref CTRL_TXT_COL: (u8, u8, u8, u8) = (255, 254, 254, 216);
    static ref DIS_CTRL_TXT_COL: (u8, u8, u8, u8) = (255, 254, 254, 63);
    static ref SEL_CTRL_COL: (u8, u8, u8, u8) = (48, 79, 120, 255);
    static ref SEL_CTRL_TXT_COL: (u8, u8, u8, u8) = (255, 254, 254, 216);
    static ref ALT_SEL_CTRL_TXT_COL: (u8, u8, u8, u8) = (255, 254, 254, 255);
    static ref SCRUB_BG_COL: (u8, u8, u8, u8) = (255, 254, 254, 255);
    static ref WIN_FRM_TXT_COL: (u8, u8, u8, u8) = (255, 254, 254, 216);
    static ref PAGE_BG_COL: (u8, u8, u8, u8) = (29, 29, 29, 255);
    static ref FIND_HLT_COL: (u8, u8, u8, u8) = (255, 255, 10, 255);
    static ref HLT_COL: (u8, u8, u8, u8) = (164, 164, 164, 255);
    static ref SHDW_COL: (u8, u8, u8, u8) = (0, 0, 0, 255);
}

fn aqua_dark_button_up_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    
}

fn aqua_dark_button_up_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    let col = &FRAME_COL;
    let svg = format!(
        "<svg width='{0}' height='{1}'>
  <defs>
    <linearGradient id='grad1' x1='0%' y1='0%' x2='0%' y2='100%'>
      <stop offset='0%' style='stop-color:rgb({2}, {3}, {4});stop-opacity:1' />
      <stop offset='100%' style='stop-color:rgb({5},{6},{7});stop-opacity:1' />
    </linearGradient>
  </defs>
  <rect width='{0}' height='{1}' rx='{8}' fill='url(#grad1)' />
    </svg>", w, h, col.0 - 10, col.1 - 10, col.2 - 10, col.0, col.1, col.2, h/3
    );
    let mut image = image::SvgImage::from_data(&svg).unwrap();
    image.draw(x, y, w, h);
    aqua_dark_button_up_frame(x, y, w, h, c);
}

fn aqua_dark_panel_thin_up_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(devalued(c, 0.06751)));
    draw_rect(x, y, w, h);
}

fn aqua_dark_panel_thin_up_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(c));
    draw_rectf(x + 1, y + 1, w - 2, h - 2);
    aqua_dark_panel_thin_up_frame(x, y, w, h, c);
}

fn aqua_dark_spacer_thin_down_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    // top and left borders
    set_draw_color(activated_color(Color::from_rgb(0xD6, 0xD6, 0xD6)));
    draw_yxline2(x, y + h - 2, y, x + w - 2);
    // bottom and right borders
    set_draw_color(activated_color(Color::from_rgb(0xF3, 0xF3, 0xF3)));
    draw_xyline2(x, y + h - 1, x + w - 1, y);
}

fn aqua_dark_spacer_thin_down_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(c));
    draw_rectf(x + 1, y + 1, w - 2, h - 2);
    aqua_dark_spacer_thin_down_frame(x, y, w, h, c);
}

fn aqua_dark_radio_round_down_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(devalued(c, 0.42194)));
    draw_arc(x, y, w, h, 0.0, 360.0);
}

fn aqua_dark_radio_round_down_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    // top edges
    set_draw_color(activated_color(Color::from_rgb(0xF6, 0xF6, 0xF6)));
    draw_arc(x + 1, y + 1, w - 2, h - 2, 0.0, 180.0);
    // bottom edges
    set_draw_color(activated_color(Color::from_rgb(0xEB, 0xEB, 0xEB)));
    draw_arc(x + 1, y + 1, w - 2, h - 2, 180.0, 360.0);
    // top gradient
    vertical_gradient(
        x + 2,
        y + 2,
        x + w - 3,
        y + h / 2 - 1,
        Color::from_rgb(0xFF, 0xFF, 0xFF),
        Color::from_rgb(0xF6, 0xF5, 0xF4),
    );
    // bottom fill
    set_draw_color(activated_color(Color::from_rgb(0xED, 0xEC, 0xEA)));
    draw_rectf(x + 2, y + h / 2, w - 4, h - h / 2 - 3);
    // bottom gradient
    set_draw_color(activated_color(Color::from_rgb(0xEF, 0xEE, 0xEC)));
    draw_xyline(x + 2, y + h - 3, x + w - 3);
    aqua_dark_radio_round_down_frame(x, y, w, h, c);
}

fn aqua_dark_depressed_down_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    // // top outer border
    // set_draw_color(activated_color(Color::from_rgb(0x4C, 0x54, 0xAA)));
    // draw_xyline(x + 3, y, x + w - 4);
    // // side outer borders
    // set_draw_color(activated_color(Color::from_rgb(0x49, 0x4C, 0x8F)));
    // draw_yxline(x, y + 3, y + h - 4);
    // draw_yxline(x + w - 1, y + 3, y + h - 4);
    // // bottom outer border
    // set_draw_color(activated_color(Color::from_rgb(0x43, 0x46, 0x72)));
    // draw_xyline(x + 3, y + h - 1, x + w - 4);
    // // top inner border
    // set_draw_color(activated_color(Color::from_rgb(0xBC, 0xD6, 0xEF)));
    // draw_xyline(x + 3, y + 1, x + w - 4);
    // // side top inner borders
    // set_draw_color(activated_color(Color::from_rgb(0x7C, 0xAB, 0xE9)));
    // draw_yxline(x + 1, y + 3, y + h / 2 - 1);
    // draw_yxline(x + w - 2, y + 3, y + h / 2 - 1);
    // // side bottom inner borders
    // set_draw_color(activated_color(Color::from_rgb(0x5F, 0xA1, 0xEA)));
    // draw_yxline(x + 1, y + h / 2, y + h - 4);
    // draw_yxline(x + w - 2, y + h / 2, y + h - 4);
    // // top corners
    // set_draw_color(activated_color(Color::from_rgb(0x79, 0x81, 0xBC)));
    // draw_arc(x, y, 8, 8, 90.0, 180.0);
    // draw_arc(x + w - 8, y, 8, 8, 0.0, 90.0);
    // // bottom corners
    // set_draw_color(activated_color(Color::from_rgb(0x72, 0x79, 0x96)));
    // draw_arc(x, y + h - 8, 8, 8, 180.0, 270.0);
    // draw_arc(x + w - 8, y + h - 8, 8, 8, 270.0, 360.0);
}

fn aqua_dark_depressed_down_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    let col = &FRAME_COL;
    let svg = format!(
        "<svg width='{0}' height='{1}'>
  <defs>
    <linearGradient id='grad1' x1='0%' y1='0%' x2='0%' y2='100%'>
      <stop offset='0%' style='stop-color:rgb({2}, {3}, {4});stop-opacity:1' />
      <stop offset='100%' style='stop-color:rgb({5},{6},{7});stop-opacity:1' />
    </linearGradient>
  </defs>
  <rect width='{0}' height='{1}' rx='{8}' fill='url(#grad1)' />
    </svg>", w, h, col.0 - 20, col.1 - 20, col.2 - 20, col.0, col.1, col.2, h/3
    );
    let mut image = image::SvgImage::from_data(&svg).unwrap();
    image.draw(x, y, w, h);
    aqua_dark_depressed_down_frame(x, y, w, h, c);
}

fn aqua_dark_input_thin_down_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    // top outer border
    set_draw_color(activated_color(Color::from_rgb(0x9B, 0x9B, 0x9B)));
    draw_xyline(x, y, x + w - 1);
    // side and bottom outer borders
    set_draw_color(activated_color(Color::from_rgb(0xBA, 0xBA, 0xBA)));
    draw_yxline3(x, y + 1, y + h - 1, x + w - 1, y + 1);
    // top shadow
    set_draw_color(activated_color(Color::from_rgb(0xE3, 0xE3, 0xE3)));
    draw_xyline(x + 1, y + 1, x + w - 2);
    // inner border
    set_draw_color(activated_color(Color::from_rgb(0xF5, 0xF5, 0xF5)));
    draw_yxline3(x + 1, y + h - 2, y + 2, x + w - 2, y + h - 2);
}

fn aqua_dark_input_thin_down_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(Color::from_rgb(0xFF, 0xFF, 0xFF)));
    draw_rectf(x + 2, y + 3, w - 4, h - 4);
    aqua_dark_input_thin_down_frame(x, y, w, h, c);
}

fn aqua_dark_default_button_up_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    // // top outer border
    // set_draw_color(activated_color(Color::from_rgb(0x4E, 0x59, 0xA6)));
    // draw_xyline(x + 3, y, x + w - 4);
    // // side outer borders
    // set_draw_color(activated_color(Color::from_rgb(0x4C, 0x52, 0x89)));
    // draw_yxline(x, y + 3, y + h - 4);
    // draw_yxline(x + w - 1, y + 3, y + h - 4);
    // // bottom outer border
    // set_draw_color(activated_color(Color::from_rgb(0x48, 0x4F, 0x69)));
    // draw_xyline(x + 3, y + h - 1, x + w - 4);
    // // top inner border
    // set_draw_color(activated_color(Color::from_rgb(0xD0, 0xEA, 0xF6)));
    // draw_xyline(x + 3, y + 1, x + w - 4);
    // // side top inner borders
    // set_draw_color(activated_color(Color::from_rgb(0x7A, 0xBF, 0xEF)));
    // draw_yxline(x + 1, y + 3, y + h / 2 - 1);
    // draw_yxline(x + w - 2, y + 3, y + h / 2 - 1);
    // // side bottom inner borders
    // set_draw_color(activated_color(Color::from_rgb(0x53, 0xAF, 0xEF)));
    // draw_yxline(x + 1, y + h / 2, y + h - 4);
    // draw_yxline(x + w - 2, y + h / 2, y + h - 4);
    // // top corners
    // set_draw_color(activated_color(Color::from_rgb(0x76, 0x80, 0xB5)));
    // draw_arc(x, y, 8, 8, 90.0, 180.0);
    // draw_arc(x + w - 8, y, 8, 8, 0.0, 90.0);
    // // bottom corners
    // set_draw_color(activated_color(Color::from_rgb(0x6F, 0x75, 0x89)));
    // draw_arc(x, y + h - 8, 8, 8, 180.0, 270.0);
    // draw_arc(x + w - 8, y + h - 8, 8, 8, 270.0, 360.0);
}

fn aqua_dark_default_button_up_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    let col = &FRAME_COL;
    let svg = format!(
        "<svg width='{0}' height='{1}'>
  <defs>
    <linearGradient id='grad1' x1='0%' y1='0%' x2='0%' y2='100%'>
      <stop offset='0%' style='stop-color:rgb({2}, {3}, {4});stop-opacity:1' />
      <stop offset='100%' style='stop-color:rgb({5},{6},{7});stop-opacity:1' />
    </linearGradient>
  </defs>
  <rect width='{0}' height='{1}' rx='{8}' fill='url(#grad1)' />
    </svg>", w, h, col.0, col.1, col.2, col.0 + 10, col.1 + 10, col.2 + 10, h/3
    );
    let mut image = image::SvgImage::from_data(&svg).unwrap();
    image.draw(x, y, w, h);
    aqua_dark_default_button_up_frame(x, y, w, h, c);
}

fn aqua_dark_tabs_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    // top outer border
    set_draw_color(activated_color(Color::from_rgb(0xAE, 0xAE, 0xAE)));
    draw_xyline(x + 3, y, x + w - 4);
    // side outer borders
    set_draw_color(activated_color(Color::from_rgb(0x9E, 0x9E, 0x9E)));
    draw_yxline(x, y + 3, y + h - 4);
    draw_yxline(x + w - 1, y + 3, y + h - 4);
    // bottom outer border
    set_draw_color(activated_color(Color::from_rgb(0x8E, 0x8E, 0x8E)));
    draw_xyline(x + 3, y + h - 1, x + w - 4);
    // top inner border
    set_draw_color(activated_color(Color::from_rgb(0xFA, 0xFA, 0xFA)));
    draw_xyline(x + 3, y + 1, x + w - 4);
    // side inner borders
    set_draw_color(activated_color(Color::from_rgb(0xF6, 0xF6, 0xF6)));
    draw_yxline(x + 1, y + 3, y + h - 4);
    draw_yxline(x + w - 2, y + 3, y + h - 4);
    // bottom inner border
    set_draw_color(activated_color(Color::from_rgb(0xF2, 0xF2, 0xF2)));
    draw_xyline(x + 3, y + h - 2, x + w - 4);
    // top corners
    set_draw_color(activated_color(Color::from_rgb(0xA4, 0xA4, 0xA4)));
    draw_arc(x, y, 8, 8, 90.0, 180.0);
    draw_arc(x + w - 8, y, 8, 8, 0.0, 90.0);
    // bottom corners
    set_draw_color(activated_color(Color::from_rgb(0x94, 0x94, 0x94)));
    draw_arc(x, y + h - 8, 8, 8, 180.0, 270.0);
    draw_arc(x + w - 8, y + h - 8, 8, 8, 270.0, 360.0);
}

fn aqua_dark_tabs_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(c));
    draw_rectf(x + 2, y + 2, w - 4, h - 4);
    aqua_dark_tabs_frame(x, y, w, h, c);
}

fn aqua_dark_swatch_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    // outer border
    set_draw_color(activated_color(Color::from_rgb(0xA3, 0xA3, 0xA3)));
    draw_rect(x, y, w, h);
    // inner border
    set_draw_color(activated_color(Color::from_rgb(0xFF, 0xFF, 0xFF)));
    draw_rect(x + 1, y + 1, w - 2, h - 2);
}

fn aqua_dark_swatch_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(c));
    draw_rectf(x + 2, y + 2, w - 4, h - 4);
    aqua_dark_swatch_frame(x, y, w, h, c);
}

fn use_aqua_dark_scheme() {
    app::set_scheme(app::Scheme::Gtk);
    app::set_frame_type_cb(OS_BUTTON_UP_BOX, aqua_dark_button_up_box, 1, 1, 2, 2);
    app::set_frame_type2(OS_CHECK_DOWN_BOX, OS_BUTTON_UP_BOX);
    app::set_frame_type_cb(OS_BUTTON_UP_FRAME, aqua_dark_button_up_frame, 1, 1, 2, 2);
    app::set_frame_type2(OS_CHECK_DOWN_FRAME, OS_BUTTON_UP_FRAME);
    app::set_frame_type_cb(
        OS_PANEL_THIN_UP_BOX,
        aqua_dark_panel_thin_up_box,
        1,
        1,
        2,
        2,
    );
    app::set_frame_type_cb(
        OS_SPACER_THIN_DOWN_BOX,
        aqua_dark_spacer_thin_down_box,
        1,
        1,
        2,
        2,
    );
    app::set_frame_type_cb(
        OS_PANEL_THIN_UP_FRAME,
        aqua_dark_panel_thin_up_frame,
        1,
        1,
        2,
        2,
    );
    app::set_frame_type_cb(
        OS_SPACER_THIN_DOWN_FRAME,
        aqua_dark_spacer_thin_down_frame,
        1,
        1,
        2,
        2,
    );
    app::set_frame_type_cb(
        OS_RADIO_ROUND_DOWN_BOX,
        aqua_dark_radio_round_down_box,
        2,
        2,
        4,
        4,
    );
    app::set_frame_type2(OS_HOVERED_UP_BOX, OS_BUTTON_UP_BOX);
    app::set_frame_type_cb(
        OS_DEPRESSED_DOWN_BOX,
        aqua_dark_depressed_down_box,
        1,
        1,
        2,
        2,
    );
    app::set_frame_type2(OS_HOVERED_UP_FRAME, OS_BUTTON_UP_FRAME);
    app::set_frame_type_cb(
        OS_DEPRESSED_DOWN_FRAME,
        aqua_dark_depressed_down_frame,
        1,
        1,
        2,
        2,
    );
    app::set_frame_type_cb(
        OS_INPUT_THIN_DOWN_BOX,
        aqua_dark_input_thin_down_box,
        2,
        3,
        4,
        6,
    );
    app::set_frame_type_cb(
        OS_INPUT_THIN_DOWN_FRAME,
        aqua_dark_input_thin_down_frame,
        2,
        3,
        4,
        6,
    );
    app::set_frame_type_cb(
        OS_DEFAULT_BUTTON_UP_BOX,
        aqua_dark_default_button_up_box,
        1,
        1,
        2,
        2,
    );
    app::set_frame_type2(OS_DEFAULT_HOVERED_UP_BOX, OS_HOVERED_UP_BOX);
    app::set_frame_type2(OS_DEFAULT_DEPRESSED_DOWN_BOX, OS_DEPRESSED_DOWN_BOX);
    app::set_frame_type2(OS_TOOLBAR_BUTTON_HOVER_BOX, FrameType::FlatBox);
    app::set_frame_type_cb(OS_TABS_BOX, aqua_dark_tabs_box, 2, 1, 4, 2);
    app::set_frame_type_cb(OS_SWATCH_BOX, aqua_dark_swatch_box, 2, 2, 4, 4);
    app::set_frame_type2(OS_MINI_BUTTON_UP_BOX, OS_BUTTON_UP_BOX);
    app::set_frame_type2(OS_MINI_DEPRESSED_DOWN_BOX, OS_DEPRESSED_DOWN_BOX);
    app::set_frame_type2(OS_MINI_BUTTON_UP_FRAME, OS_BUTTON_UP_FRAME);
    app::set_frame_type2(OS_MINI_DEPRESSED_DOWN_FRAME, OS_DEPRESSED_DOWN_FRAME);
    app::set_frame_type2(FrameType::UpBox, OS_BUTTON_UP_BOX);
    app::set_frame_type2(FrameType::DownBox, OS_BUTTON_UP_BOX);
    app::set_frame_type2(FrameType::RoundDownBox, OS_RADIO_ROUND_DOWN_BOX);
    app::set_frame_type2(OS_BG_BOX, FrameType::FlatBox);
    // app::set_frame_type_cb(OS_BG_DOWN_BOX, OS_BG_BOX);
}

fn use_aqua_dark_colors() {
    app::background(BG_COL.0, BG_COL.1, BG_COL.2);
    app::foreground(FG_COL.0, FG_COL.1, FG_COL.2);
    app::background2(BG2_COL.0, BG2_COL.1, BG2_COL.2);
    app::set_color(Color::Inactive, 0x4D, 0x4D, 0x69);
    app::set_color(Color::Selection, 0x30, 0x60, 0xF6);
    app::set_color(Color::Free, 0xFB, 0xFB, 0xFB);
    Tooltip::set_color(Color::from_rgb(0xFF, 0xFF, 0xC7));
    Tooltip::set_text_color(Color::ForeGround);
}

pub(crate) fn use_aqua_dark_theme() {
    use_aqua_dark_scheme();
    use_aqua_dark_colors();
    use_native_settings();
}
