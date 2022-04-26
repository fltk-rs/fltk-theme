use fltk::{
    app, draw,
    enums::{Color, FrameType},
    image,
    prelude::ImageExt,
};

fn up_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    let col1 = c.to_rgb();
    let col = Color::color_average(c, Color::Background, 0.8).to_rgb();
    let svg = format!(
        "<svg viewBox='0 0 {0} {1}'>
  <defs>
    <linearGradient id='grad1' x1='0%' y1='0%' x2='0%' y2='100%'>
      <stop offset='0%' style='stop-color:rgb({2},{3},{4});stop-opacity:1' />
      <stop offset='100%' style='stop-color:rgb({5},{6},{7});stop-opacity:1' />
    </linearGradient>
  </defs>
  <rect width='98%' height='98%' rx='5' fill='url(#grad1)' />
    </svg>",
        w,
        h,
        col.0,
        col.1,
        col.2,
        col1.0,
        col1.1,
        col1.2,
    );
    let mut image = image::SvgImage::from_data(&svg).unwrap();
    image.draw(x, y, w, h);
}

fn default_button_up_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
  let col1 = c.to_rgb();
  let col = Color::color_average(c, Color::Background, 0.8).to_rgb();
    let svg = format!(
        "<svg viewBox='0 0 {0} {1}'>
  <defs>
    <linearGradient id='grad1' x1='0%' y1='0%' x2='0%' y2='100%'>
      <stop offset='0%' style='stop-color:rgb({2},{3},{4});stop-opacity:1' />
      <stop offset='100%' style='stop-color:rgb({5},{6},{7});stop-opacity:1' />
    </linearGradient>s
  </defs>
  <rect width='98%' height='98%' rx='5' fill='url(#grad1)' />
    </svg>",
        w,
        h,
        col.0,
        col.1,
        col.2,
        col1.0,
        col1.1,
        col1.2,
    );
    let mut image = image::SvgImage::from_data(&svg).unwrap();
    image.draw(x, y, w, h);
}

fn down_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
  let col1 = c.to_rgb();
  let col = Color::color_average(c, Color::Background, 0.8).to_rgb();
    let svg = format!(
        "<svg viewBox='0 0 {0} {1}'>
  <defs>
    <linearGradient id='grad1' x1='0%' y1='0%' x2='0%' y2='100%'>
      <stop offset='0%' style='stop-color:rgb({2},{3},{4});stop-opacity:1' />
      <stop offset='100%' style='stop-color:rgb({5},{6},{7});stop-opacity:1' />
    </linearGradient>
  </defs>
  <rect width='98%' height='98%' rx='5' fill='url(#grad1)' />
    </svg>",
        w,
        h,
        col.0,
        col.1,
        col.2,
        col1.0,
        col1.1,
        col1.2,
    );
    let mut image = image::SvgImage::from_data(&svg).unwrap();
    image.draw(x, y, w, h);
}

fn radio_round_down_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    let col = c.to_rgb();
    let svg = format!(
        "<svg viewBox='0 0 {} {}'>
  <circle cx='50%' cy='48%' r='48%' fill='rgb({},{},{})'/>
    </svg>",
        w,
        h,
        col.0,
        col.1,
        col.2
    );
    let mut image = image::SvgImage::from_data(&svg).unwrap();
    image.draw(x, y, w, h);
}

fn border_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    draw::draw_box(
        FrameType::RFlatBox,
        x + 1,
        y + 1,
        w - 2,
        h - 2,
        *crate::colors::aqua::dark::systemBlueColor,
    );
}

fn use_scheme() {
    app::set_scheme(app::Scheme::Gtk);
    // app::set_menu_linespacing(5);
    app::set_frame_type_cb(FrameType::UpBox, up_box, 2, 2, 4, 4);
    app::set_frame_type_cb(FrameType::DiamondUpBox, default_button_up_box, 2, 2, 4, 4);
    app::set_frame_type_cb(FrameType::DownBox, down_box, 2, 2, 4, 4);
    app::set_frame_type_cb(FrameType::DiamondDownBox, down_box, 2, 2, 4, 4);
    app::set_frame_type_cb(FrameType::RoundDownBox, radio_round_down_box, 2, 2,4, 4);
    app::set_frame_type_cb(FrameType::BorderBox, border_box, 2, 2, 4, 4);
}

pub(crate) fn use_aqua_scheme() {
    use_scheme();
    app::set_visible_focus(false);
    app::set_scrollbar_size(15);
}

pub mod frames {
    use fltk::enums::FrameType::{self, *};
    pub const OS_DEFAULT_BUTTON_UP_BOX: FrameType = DiamondUpBox;
    pub const OS_DEFAULT_DEPRESSED_DOWN_BOX: FrameType = FrameType::DiamondDownBox;
}
