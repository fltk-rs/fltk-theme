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
    static ref SYS_CYAN: (u8, u8, u8, u8) = (90, 200 , 245, 255); //beta!
    static ref BG_COL: (u8, u8, u8, u8) = get_colors!(my_windowBackgroundColor);
    static ref FG_COL: (u8, u8, u8, u8) = get_colors!(my_labelColor);
    static ref CTRL_BG_COL: (u8, u8, u8, u8) = get_colors!(my_controlBackgroundColor);
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
    static ref SYS_CYAN: (u8, u8, u8, u8) = (90, 200 , 245, 255);
    static ref BG_COL: (u8, u8, u8, u8) = (37, 37, 37, 255);
    static ref FG_COL: (u8, u8, u8, u8) = (255, 254, 254, 216);
    static ref CTRL_BG_COL: (u8, u8, u8, u8) = (22, 22, 22, 255);
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
    </svg>", w, h, col.0 - 10, col.1 - 10, col.2 - 10, col.0, col.1, col.2, h/4
    );
    let mut image = image::SvgImage::from_data(&svg).unwrap();
    image.draw(x, y, w, h);
}

fn aqua_dark_button_down_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
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
    </svg>", w, h, col.0 - 10, col.1 - 10, col.2 - 10, col.0, col.1, col.2, h/4
    );
    let mut image = image::SvgImage::from_data(&svg).unwrap();
    image.draw(x, y, w, h);
}

fn aqua_dark_depressed_down_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    let col = &SYS_CYAN;
    let svg = format!(
        "<svg width='{0}' height='{1}'>
  <defs>
    <linearGradient id='grad1' x1='0%' y1='0%' x2='0%' y2='100%'>
      <stop offset='0%' style='stop-color:rgb({2}, {3}, {4});stop-opacity:1' />
      <stop offset='100%' style='stop-color:rgb({5},{6},{7});stop-opacity:1' />
    </linearGradient>
  </defs>
  <rect width='{0}' height='{1}' rx='{8}' fill='url(#grad1)' />
    </svg>", w, h, col.0 - 10, col.1 - 10, col.2 - 10, col.0, col.1, col.2, h/4
    );
    let mut image = image::SvgImage::from_data(&svg).unwrap();
    image.draw(x, y, w, h);
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
    </svg>", w, h, col.0, col.1, col.2, col.0 + 10, col.1 + 10, col.2 + 10, h/4
    );
    let mut image = image::SvgImage::from_data(&svg).unwrap();
    image.draw(x, y, w, h);
}

fn aqua_dark_radio_round_down_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    let col = &CTRL_ACC_COL;
    let svg = format!(
        "<svg width='{}' height='{}'>
  <circle cx='{}' cy='{}' r='{}' fill='rgb({},{},{})'/>
    </svg>", w, h, w/2, h/2, w/2, col.0, col.1, col.2
    );
    let mut image = image::SvgImage::from_data(&svg).unwrap();
    image.draw(x, y, w, h);
}

fn use_aqua_dark_scheme() {
    app::set_scheme(app::Scheme::Gtk);
    app::set_frame_type_cb(FrameType::UpBox, aqua_dark_button_up_box, 1, 1, 2, 2);
    app::set_frame_type_cb(FrameType::DownBox, aqua_dark_button_down_box, 1, 1, 2, 2);
    app::set_frame_type_cb(
        OS_DEFAULT_BUTTON_UP_BOX,
        aqua_dark_default_button_up_box,
        1,
        1,
        2,
        2,
    );
    app::set_frame_type_cb(
        OS_DEFAULT_DEPRESSED_DOWN_BOX,
        aqua_dark_depressed_down_box,
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
    app::set_frame_type2(FrameType::RoundDownBox, OS_RADIO_ROUND_DOWN_BOX);
    app::set_frame_type2(OS_BG_BOX, FrameType::FlatBox);
    // app::set_frame_type_cb(OS_BG_DOWN_BOX, OS_BG_BOX);
}

fn use_aqua_dark_colors() {
    app::background(BG_COL.0, BG_COL.1, BG_COL.2);
    app::foreground(FG_COL.0, FG_COL.1, FG_COL.2);
    app::background2(BG2_COL.0, BG2_COL.1, BG2_COL.2);
    app::set_color(Color::Inactive, 0x4D, 0x4D, 0x69);
    app::set_color(Color::Selection, 255, 255, 255);
    app::set_color(Color::Free, 0xFB, 0xFB, 0xFB);
    Tooltip::set_color(Color::from_rgb(0xFF, 0xFF, 0xC7));
    Tooltip::set_text_color(Color::ForeGround);
}

pub(crate) fn use_aqua_dark_theme() {
    use_aqua_dark_scheme();
    use_aqua_dark_colors();
    use_native_settings();
}
