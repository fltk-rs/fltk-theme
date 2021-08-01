use super::*;
use fltk::prelude::ImageExt;

fn rounded_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    let (r, g, b) = c.to_rgb();
    let svg = format!("<?xml version='1.0'?><svg width='{}' height='{}' xmlns='http://www.w3.org/2000/svg'>
    <rect stroke-width='1' rx='15' width='{}' height='{}' stroke='rgb({}, {}, {})' fill='none' y='1' x='1'/>
  </svg>",w, h, w - 2, h - 2, r, g, b);
    let mut image = fltk::image::SvgImage::from_data(&svg).unwrap();
    image.draw(x, y, w, h);
}

fn rounded_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    let (r, g, b) = c.to_rgb();
    let svg = format!("<?xml version='1.0'?><svg width='{}' height='{}' xmlns='http://www.w3.org/2000/svg'>
    <rect stroke-width='1' stroke='black' rx='15' width='{}' height='{}' fill='rgb({}, {}, {})' y='1' x='1'/>
  </svg>",w, h, w - 2, h - 2, r, g, b);
    let mut image = fltk::image::SvgImage::from_data(&svg).unwrap();
    image.draw(x, y, w, h);
}

fn rflat_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    let (r, g, b) = c.to_rgb();
    let svg = format!("<?xml version='1.0'?><svg width='{}' height='{}' xmlns='http://www.w3.org/2000/svg'>
    <rect stroke-width='1' stroke='none' rx='15' width='{}' height='{}' fill='rgb({}, {}, {})' y='1' x='1'/>
  </svg>",w, h, w - 2, h - 2, r, g, b);
    let mut image = fltk::image::SvgImage::from_data(&svg).unwrap();
    image.draw(x, y, w, h);
}

fn rshadow_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    let (r, g, b) = c.to_rgb();
    let svg = format!("<?xml version='1.0'?><svg width='{}' height='{}' xmlns='http://www.w3.org/2000/svg'>
    <rect stroke-width='1' stroke='black' rx='15' width='{}' height='{}' fill='black' y='2' x='2'/>
    <rect stroke-width='1' stroke='black' rx='15' width='{}' height='{}' fill='rgb({}, {}, {})' y='1' x='1'/>
  </svg>",w, h, w - 2 , h - 2 , w - 2, h -2, r, g, b);
    let mut image = fltk::image::SvgImage::from_data(&svg).unwrap();
    image.draw(x, y, w, h);
}

fn round_up_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    let (r, g, b) = c.to_rgb();
    let svg = format!("<?xml version='1.0'?><svg width='{}' height='{}' xmlns='http://www.w3.org/2000/svg'>
    <rect stroke-width='1' stroke='black' rx='35' width='{}' height='{}' fill='black' y='2' x='2'/>
    <rect stroke-width='1' stroke='black' rx='35' width='{}' height='{}' fill='rgb({}, {}, {})' y='1' x='1'/>
  </svg>",w, h, w - 2 , h - 2 , w - 2, h -2, r, g, b);
    let mut image = fltk::image::SvgImage::from_data(&svg).unwrap();
    image.draw(x, y, w, h);
}

fn round_down_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    let (r, g, b) = c.to_rgb();
    let svg = format!("<?xml version='1.0'?><svg width='{}' height='{}' xmlns='http://www.w3.org/2000/svg'>
    <rect stroke-width='1' stroke='black' rx='35' width='{}' height='{}' fill='black' y='1' x='1'/>
    <rect stroke-width='1' stroke='black' rx='35' width='{}' height='{}' fill='rgb({}, {}, {})' y='2' x='2'/>
  </svg>",w, h, w -2, h - 2 , w - 2, h -2, r, g, b);
    let mut image = fltk::image::SvgImage::from_data(&svg).unwrap();
    image.draw(x, y, w, h);
}

fn oval_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    let (r, g, b) = c.to_rgb();
    let svg = format!("<?xml version='1.0'?><svg width='{}' height='{}' xmlns='http://www.w3.org/2000/svg'>
    <rect stroke-width='1' stroke='black' rx='90' width='{}' height='{}' fill='rgb({}, {}, {})' y='1' x='1'/>
  </svg>",w, h, w - 2, h - 2, r, g, b);
    let mut image = fltk::image::SvgImage::from_data(&svg).unwrap();
    image.draw(x, y, w, h);
}

fn oval_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    let (r, g, b) = c.to_rgb();
    let svg = format!("<?xml version='1.0'?><svg width='{}' height='{}' xmlns='http://www.w3.org/2000/svg'>
    <rect stroke-width='1' rx='90' width='{}' height='{}' stroke='rgb({}, {}, {})' fill='none' y='1' x='1'/>
  </svg>",w, h, w - 2, h - 2, r, g, b);
    let mut image = fltk::image::SvgImage::from_data(&svg).unwrap();
    image.draw(x, y, w, h);
}

fn oflat_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    let (r, g, b) = c.to_rgb();
    let svg = format!("<?xml version='1.0'?><svg width='{}' height='{}' xmlns='http://www.w3.org/2000/svg'>
    <rect stroke-width='1' stroke='none' rx='90' width='{}' height='{}' fill='rgb({}, {}, {})' y='1' x='1'/>
  </svg>",w, h, w - 2, h - 2, r, g, b);
    let mut image = fltk::image::SvgImage::from_data(&svg).unwrap();
    image.draw(x, y, w, h);
}

fn oshadow_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    let (r, g, b) = c.to_rgb();
    let svg = format!("<?xml version='1.0'?><svg width='{}' height='{}' xmlns='http://www.w3.org/2000/svg'>
    <rect stroke-width='1' stroke='black' rx='90' width='{}' height='{}' fill='black' y='2' x='2'/>
    <rect stroke-width='1' stroke='black' rx='90' width='{}' height='{}' fill='rgb({}, {}, {})' y='1' x='1'/>
  </svg>",w, h, w - 2 , h - 2 , w - 2, h -2, r, g, b);
    let mut image = fltk::image::SvgImage::from_data(&svg).unwrap();
    image.draw(x, y, w, h);
}


pub(crate) fn use_svg_round_scheme() {
    use fltk::enums::FrameType::*;
    app::reload_scheme().ok();
    app::set_scheme(app::Scheme::Base);
    app::set_frame_type_cb(RoundedFrame, rounded_frame, 1, 1, 2, 2);
    app::set_frame_type_cb(RoundedBox, rounded_box, 1, 1, 2, 2);
    app::set_frame_type_cb(RFlatBox, rflat_box, 1, 1, 2, 2);
    app::set_frame_type_cb(RShadowBox, rshadow_box, 1, 1, 2, 2);
    app::set_frame_type_cb(RoundUpBox, round_up_box, 1, 1, 2, 2);
    app::set_frame_type_cb(RoundDownBox, round_down_box, 1, 1, 2, 2);
    app::set_frame_type_cb(OvalBox, oval_box, 1, 1, 2, 2);
    app::set_frame_type_cb(OvalFrame, oval_frame, 1, 1, 2, 2);
    app::set_frame_type_cb(OFlatFrame, oflat_box, 1, 1, 2, 2);
    app::set_frame_type_cb(OShadowBox, oshadow_box, 1, 1, 2, 2);
}
