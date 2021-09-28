#[cfg(target_os = "macos")]
use crate::cocoa_helper::*;
use fltk::{app, enums::{Color, FrameType}, image, prelude::ImageExt};

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
mod sys {
    pub use crate::FromColor;
    lazy_static::lazy_static! {
        pub static ref BG_COL: (u8, u8, u8, u8) = get_colors!(my_windowBackgroundColor);
        pub static ref FG_COL: (u8, u8, u8, u8) = get_colors!(my_labelColor);
        pub static ref CTRL_BG_COL: (u8, u8, u8, u8) = get_colors!(my_controlBackgroundColor);
        pub static ref FRAME_COL: (u8, u8, u8, u8) = get_colors!(my_windowFrameColor);
        pub static ref LABEL2_COL: (u8, u8, u8, u8) = get_colors!(my_secondaryLabelColor);
        pub static ref LABEL3_COL: (u8, u8, u8, u8) = get_colors!(my_tertiaryLabelColor);
        pub static ref LABEL4_COL: (u8, u8, u8, u8) = get_colors!(my_quaternaryLabelColor);
        pub static ref TXT_COL: (u8, u8, u8, u8) = get_colors!(my_textColor);
        pub static ref PH_TXT_COL: (u8, u8, u8, u8) = get_colors!(my_placeholderTextColor);
        pub static ref SEL_TXT_COL: (u8, u8, u8, u8) = get_colors!(my_selectedTextColor);
        pub static ref TXT_BG_COL: (u8, u8, u8, u8) = get_colors!(my_textBackgroundColor);
        pub static ref SEL_TXT_BG_COL: (u8, u8, u8, u8) = get_colors!(my_selectedTextBackgroundColor);
        pub static ref KB_IND_COL: (u8, u8, u8, u8) = get_colors!(my_keyboardFocusIndicatorColor);
        pub static ref SEL_TXT2_COL: (u8, u8, u8, u8) = get_colors!(my_unemphasizedSelectedTextColor);
        pub static ref SEL_TXT_BG2_COL: (u8, u8, u8, u8) = get_colors!(my_unemphasizedSelectedTextBackgroundColor);
        pub static ref LINK_COL: (u8, u8, u8, u8) = get_colors!(my_linkColor);
        pub static ref SEP_COL: (u8, u8, u8, u8) = get_colors!(my_separatorColor);
        pub static ref SEL_BG_COL: (u8, u8, u8, u8) = get_colors!(my_selectedContentBackgroundColor);
        pub static ref SEL_BG2_COL: (u8, u8, u8, u8) = get_colors!(my_unemphasizedSelectedContentBackgroundColor);
        pub static ref SEL_MEN_TXT_COL: (u8, u8, u8, u8) = get_colors!(my_selectedMenuItemTextColor);
        pub static ref GRID_COL: (u8, u8, u8, u8) = get_colors!(my_gridColor);
        pub static ref HDR_COL: (u8, u8, u8, u8) = get_colors!(my_headerTextColor);
        pub static ref CTRL_ACC_COL: (u8, u8, u8, u8) = get_colors!(my_controlAccentColor);
        pub static ref CTRL_COL: (u8, u8, u8, u8) = get_colors!(my_controlColor);
        pub static ref CTRL_TXT_COL: (u8, u8, u8, u8) = get_colors!(my_controlTextColor);
        pub static ref DIS_CTRL_TXT_COL: (u8, u8, u8, u8) = get_colors!(my_disabledControlTextColor);
        pub static ref SEL_CTRL_COL: (u8, u8, u8, u8) = get_colors!(my_selectedControlColor);
        pub static ref SEL_CTRL_TXT_COL: (u8, u8, u8, u8) = get_colors!(my_selectedControlTextColor);
        pub static ref ALT_SEL_CTRL_TXT_COL: (u8, u8, u8, u8) = get_colors!(my_alternateSelectedControlTextColor);
        pub static ref SCRUB_BG_COL: (u8, u8, u8, u8) = get_colors!(my_scrubberTexturedBackgroundColor);
        pub static ref WIN_FRM_TXT_COL: (u8, u8, u8, u8) = get_colors!(my_windowFrameTextColor);
        pub static ref PAGE_BG_COL: (u8, u8, u8, u8) = get_colors!(my_underPageBackgroundColor);
        pub static ref FIND_HLT_COL: (u8, u8, u8, u8) = get_colors!(my_findHighlightColor);
        pub static ref HLT_COL: (u8, u8, u8, u8) = get_colors!(my_highlightColor);
        pub static ref SHDW_COL: (u8, u8, u8, u8) = get_colors!(my_shadowColor);
    }
}

pub mod dark {
    pub use crate::FromColor;
    lazy_static::lazy_static! {
        pub static ref BG2_COL: (u8, u8, u8, u8) = (0, 0, 0, 255);
        pub static ref SYS_CYAN: (u8, u8, u8, u8) = (90, 200 , 245, 255);
        pub static ref BG_COL: (u8, u8, u8, u8) = (37, 37, 37, 255);
        pub static ref FG_COL: (u8, u8, u8, u8) = (255, 254, 254, 216);
        pub static ref CTRL_BG_COL: (u8, u8, u8, u8) = (22, 22, 22, 255);
        pub static ref FRAME_COL: (u8, u8, u8, u8) = (153, 153, 153, 255);
        pub static ref LABEL2_COL: (u8, u8, u8, u8) = (255, 254, 254, 140);
        pub static ref LABEL3_COL: (u8, u8, u8, u8) = (255, 254, 254, 63);
        pub static ref LABEL4_COL: (u8, u8, u8, u8) = (255, 254, 254, 25);
        pub static ref TXT_COL: (u8, u8, u8, u8) = (255, 254, 254, 255);
        pub static ref PH_TXT_COL: (u8, u8, u8, u8) = (255, 254, 254, 63);
        pub static ref SEL_TXT_COL: (u8, u8, u8, u8) = (255, 255, 255, 255);
        pub static ref TXT_BG_COL: (u8, u8, u8, u8) = (22, 22, 22, 255);
        pub static ref SEL_TXT_BG_COL: (u8, u8, u8, u8) = (48, 79, 120, 255);
        pub static ref KB_IND_COL: (u8, u8, u8, u8) = (27, 149, 254, 76);
        pub static ref SEL_TXT2_COL: (u8, u8, u8, u8) = (255, 255, 255, 255);
        pub static ref SEL_TXT_BG2_COL: (u8, u8, u8, u8) = (54, 54, 54, 255);
        pub static ref LINK_COL: (u8, u8, u8, u8) = (52, 134, 254, 255);
        pub static ref SEP_COL: (u8, u8, u8, u8) = (255, 254, 254, 25);
        pub static ref SEL_BG_COL: (u8, u8, u8, u8) = (5, 63, 197, 255);
        pub static ref SEL_BG2_COL: (u8, u8, u8, u8) = (54, 54, 54, 255);
        pub static ref SEL_MEN_TXT_COL: (u8, u8, u8, u8) = (255, 254, 254, 255);
        pub static ref GRID_COL: (u8, u8, u8, u8) = (20, 20, 20, 255);
        pub static ref HDR_COL: (u8, u8, u8, u8) = (255, 254, 254, 255);
        pub static ref CTRL_ACC_COL: (u8, u8, u8, u8) = (10, 95, 254, 255);
        pub static ref CTRL_COL: (u8, u8, u8, u8) = (255, 254, 254, 63);
        pub static ref CTRL_TXT_COL: (u8, u8, u8, u8) = (255, 254, 254, 216);
        pub static ref DIS_CTRL_TXT_COL: (u8, u8, u8, u8) = (255, 254, 254, 63);
        pub static ref SEL_CTRL_COL: (u8, u8, u8, u8) = (48, 79, 120, 255);
        pub static ref SEL_CTRL_TXT_COL: (u8, u8, u8, u8) = (255, 254, 254, 216);
        pub static ref ALT_SEL_CTRL_TXT_COL: (u8, u8, u8, u8) = (255, 254, 254, 255);
        pub static ref SCRUB_BG_COL: (u8, u8, u8, u8) = (255, 254, 254, 255);
        pub static ref WIN_FRM_TXT_COL: (u8, u8, u8, u8) = (255, 254, 254, 216);
        pub static ref PAGE_BG_COL: (u8, u8, u8, u8) = (29, 29, 29, 255);
        pub static ref FIND_HLT_COL: (u8, u8, u8, u8) = (255, 255, 10, 255);
        pub static ref HLT_COL: (u8, u8, u8, u8) = (164, 164, 164, 255);
        pub static ref SHDW_COL: (u8, u8, u8, u8) = (0, 0, 0, 255);
    }
}

pub mod light {
    pub use crate::FromColor;
    lazy_static::lazy_static! {
       pub static ref BG2_COL: (u8, u8, u8, u8) = (255, 255, 255, 255);
       pub static ref SYS_CYAN: (u8, u8, u8, u8) = (85, 190 , 240, 255);
       pub static ref BG_COL: (u8, u8, u8, u8) = (231, 231, 231, 255);
       pub static ref FG_COL: (u8, u8, u8, u8) = (0, 0, 0, 216);
       pub static ref CTRL_BG_COL: (u8, u8, u8, u8) = (255, 254, 254, 255);
       pub static ref FRAME_COL: (u8, u8, u8, u8) = (153, 153, 153, 255);
       pub static ref LABEL2_COL: (u8, u8, u8, u8) = (0, 0, 0, 127);
       pub static ref LABEL3_COL: (u8, u8, u8, u8) = (0, 0, 0, 66);
       pub static ref LABEL4_COL: (u8, u8, u8, u8) = (0, 0, 0, 25);
       pub static ref TXT_COL: (u8, u8, u8, u8) = (0, 0, 0, 255);
       pub static ref PH_TXT_COL: (u8, u8, u8, u8) = (0, 0, 0, 63);
       pub static ref SEL_TXT_COL: (u8, u8, u8, u8) = (0, 0, 0, 255);
       pub static ref TXT_BG_COL: (u8, u8, u8, u8) = (255, 254, 254, 255);
       pub static ref SEL_TXT_BG_COL: (u8, u8, u8, u8) = (164, 204, 254, 255);
       pub static ref KB_IND_COL: (u8, u8, u8, u8) = (7, 75, 240, 63);
       pub static ref SEL_TXT2_COL: (u8, u8, u8, u8) = (0, 0, 0, 255);
       pub static ref SEL_TXT_BG2_COL: (u8, u8, u8, u8) = (211, 211, 211, 255);
       pub static ref LINK_COL: (u8, u8, u8, u8) = (8, 79, 209, 255);
       pub static ref SEP_COL: (u8, u8, u8, u8) = (0, 0, 0, 25);
       pub static ref SEL_BG_COL: (u8, u8, u8, u8) = (7, 73, 217, 255);
       pub static ref SEL_BG2_COL: (u8, u8, u8, u8) = (211, 211, 211, 255);
       pub static ref SEL_MEN_TXT_COL: (u8, u8, u8, u8) = (255, 254, 254, 255);
       pub static ref GRID_COL: (u8, u8, u8, u8) = (223, 223, 223, 255);
       pub static ref HDR_COL: (u8, u8, u8, u8) = (0, 0, 0, 216);
       pub static ref CTRL_ACC_COL: (u8, u8, u8, u8) = (10, 95, 254, 255);
       pub static ref CTRL_COL: (u8, u8, u8, u8) = (255, 254, 254, 255);
       pub static ref CTRL_TXT_COL: (u8, u8, u8, u8) = (0, 0, 0, 216);
       pub static ref DIS_CTRL_TXT_COL: (u8, u8, u8, u8) = (0, 0, 0, 63);
       pub static ref SEL_CTRL_COL: (u8, u8, u8, u8) = (164, 204, 254, 255);
       pub static ref SEL_CTRL_TXT_COL: (u8, u8, u8, u8) = (0, 0, 0, 216);
       pub static ref ALT_SEL_CTRL_TXT_COL: (u8, u8, u8, u8) = (255, 254, 254, 255);
       pub static ref SCRUB_BG_COL: (u8, u8, u8, u8) = (255, 254, 254, 255);
       pub static ref WIN_FRM_TXT_COL: (u8, u8, u8, u8) = (0, 0, 0, 216);
       pub static ref PAGE_BG_COL: (u8, u8, u8, u8) = (131, 131, 131, 229);
       pub static ref FIND_HLT_COL: (u8, u8, u8, u8) = (255, 255, 10, 255);
       pub static ref HLT_COL: (u8, u8, u8, u8) = (255, 254, 254, 255);
       pub static ref SHDW_COL: (u8, u8, u8, u8) = (0, 0, 0, 255);
    }
}

fn button_up_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    let col = c.to_rgb();
    let bg = Color::BackGround.to_rgb();
    let svg = format!(
        "<svg width='{0}' height='{1}'>
  <defs>
    <linearGradient id='grad1' x1='0%' y1='0%' x2='0%' y2='100%'>
      <stop offset='0%' style='stop-color:rgb({2},{3},{4});stop-opacity:{5}' />
      <stop offset='100%' style='stop-color:rgb({6},{7},{8});stop-opacity:{5}' />
    </linearGradient>
  </defs>
  <rect width='{0}' height='{1}' rx='{9}' fill='rgb({10},{11},{12})' />
  <rect width='{0}' height='{1}' rx='{9}' fill='url(#grad1)' />
    </svg>",
        w,
        h,
        col.0,
        col.1,
        col.2,
        if bg.0 > 230 && bg.1 > 230 && bg.2 > 230 {
            1.0
        } else {
            63 as f64 / 255.0
        },
        col.0 - 10,
        col.1 - 10,
        col.2 - 10,
        h / 4,
        bg.0,
        bg.1,
        bg.2
    );
    let mut image = image::SvgImage::from_data(&svg).unwrap();
    image.draw(x, y, w, h);
}

fn depressed_down_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    let col = c.to_rgb();
    let svg = format!(
        "<svg width='{0}' height='{1}'>
  <defs>
    <linearGradient id='grad1' x1='0%' y1='0%' x2='0%' y2='100%'>
      <stop offset='0%' style='stop-color:rgb({2},{3},{4});stop-opacity:1' />
      <stop offset='100%' style='stop-color:rgb({5},{6},{7});stop-opacity:1' />
    </linearGradient>
  </defs>
  <rect width='{0}' height='{1}' rx='{8}' fill='url(#grad1)' />
    </svg>",
        w,
        h,
        col.0 - 10,
        col.1 - 10,
        col.2 - 10,
        col.0,
        col.1,
        col.2,
        h / 4
    );
    let mut image = image::SvgImage::from_data(&svg).unwrap();
    image.draw(x, y, w, h);
}

fn radio_round_down_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    let col = c.to_rgb();
    let svg = format!(
        "<svg width='{}' height='{}'>
  <circle cx='{}' cy='{}' r='{}' fill='rgb({},{},{})'/>
    </svg>",
        w,
        h,
        w / 2,
        h / 2,
        (w as f64 - 1.0) / 2.0,
        col.0,
        col.1,
        col.2
    );
    let mut image = image::SvgImage::from_data(&svg).unwrap();
    image.draw(x, y, w, h);
}

fn use_scheme() {
    app::set_scheme(app::Scheme::Gtk);
    app::set_frame_type_cb(FrameType::UpBox, button_up_box, 1, 1, 2, 2);
    app::set_frame_type_cb(
        FrameType::DownBox,
        depressed_down_box,
        1,
        1,
        2,
        2,
    );
    app::set_frame_type_cb(
        FrameType::RoundDownBox,
        radio_round_down_box,
        2,
        2,
        4,
        4,
    );
    // app::set_frame_type_cb(OS_BG_DOWN_BOX, OS_BG_BOX);
}

pub(crate) fn use_aqua_scheme() {
    use_scheme();
    app::set_visible_focus(false);
    app::set_scrollbar_size(15);
}
