use super::*;
use fltk::prelude::ImageExt;

fn rounded_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    let (r, g, b) = c.to_rgb();
    let svg = format!("<?xml version='1.0'?><svg width='{}' height='{}' xmlns='http://www.w3.org/2000/svg'>
    <rect stroke-width='2' rx='15' width='{}' height='{}' stroke='rgb({}, {}, {})' fill='none' y='1' x='1'/>
  </svg>",w, h, w - 2, h - 2, r, g, b);
    let mut image = fltk::image::SvgImage::from_data(&svg).unwrap();
    image.draw(x, y, w, h);
}

fn rounded_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    let (r, g, b) = c.to_rgb();
    let svg = format!("<?xml version='1.0'?><svg width='{}' height='{}' xmlns='http://www.w3.org/2000/svg'>
    <rect stroke-width='2' stroke='rgb(200, 200, 200)' rx='15' width='{}' height='{}' fill='rgb({}, {}, {})' y='1' x='1'/>
  </svg>",w, h, w - 2, h - 2, r, g, b);
    let mut image = fltk::image::SvgImage::from_data(&svg).unwrap();
    image.draw(x, y, w, h);
}

fn rflat_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    let (r, g, b) = c.to_rgb();
    let svg = format!("<?xml version='1.0'?><svg width='{}' height='{}' xmlns='http://www.w3.org/2000/svg'>
    <rect stroke-width='2' stroke='none' rx='15' width='{}' height='{}' fill='rgb({}, {}, {})' y='1' x='1'/>
  </svg>",w, h, w - 2, h - 2, r, g, b);
    let mut image = fltk::image::SvgImage::from_data(&svg).unwrap();
    image.draw(x, y, w, h);
}

fn oval_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    let (r, g, b) = c.to_rgb();
    let svg = format!("<?xml version='1.0'?><svg width='{}' height='{}' xmlns='http://www.w3.org/2000/svg'>
    <rect stroke-width='2' stroke='rgb(200, 200, 200)' rx='90' width='{}' height='{}' fill='rgb({}, {}, {})' y='1' x='1'/>
  </svg>",w, h, w - 2, h - 2, r, g, b);
    let mut image = fltk::image::SvgImage::from_data(&svg).unwrap();
    image.draw(x, y, w, h);
}

fn oval_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    let (r, g, b) = c.to_rgb();
    let svg = format!("<?xml version='1.0'?><svg width='{}' height='{}' xmlns='http://www.w3.org/2000/svg'>
    <rect stroke-width='2' rx='90' width='{}' height='{}' stroke='rgb({}, {}, {})' fill='none' y='1' x='1'/>
  </svg>",w, h, w - 2, h - 2, r, g, b);
    let mut image = fltk::image::SvgImage::from_data(&svg).unwrap();
    image.draw(x, y, w, h);
}

fn oflat_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    let (r, g, b) = c.to_rgb();
    let svg = format!("<?xml version='1.0'?><svg width='{}' height='{}' xmlns='http://www.w3.org/2000/svg'>
    <rect stroke-width='2' stroke='none' rx='90' width='{}' height='{}' fill='rgb({}, {}, {})' y='1' x='1'/>
  </svg>",w, h, w - 2, h - 2, r, g, b);
    let mut image = fltk::image::SvgImage::from_data(&svg).unwrap();
    image.draw(x, y, w, h);
}


pub(crate) fn use_svg_based_scheme() {
    use fltk::enums::FrameType::*;
    app::reload_scheme().ok();
    app::set_scheme(app::Scheme::Base);
    app::set_frame_type_cb(RoundedFrame, rounded_frame, 0, 0, 0, 0);
    app::set_frame_type_cb(RoundedBox, rounded_box, 0, 0, 0, 0);
    app::set_frame_type_cb(RFlatBox, rflat_box, 0, 0, 0, 0);
    app::set_frame_type_cb(OvalBox, oval_box, 0, 0, 0, 0);
    app::set_frame_type_cb(OvalFrame, oval_frame, 0, 0, 0, 0);
    app::set_frame_type_cb(OFlatFrame, oflat_box, 0, 0, 0, 0);
}
