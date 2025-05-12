use super::*;

fn scheme1_up_box_draw(x: i32, y: i32, w: i32, h: i32, c: Color) {
    draw_rect_fill(x + 1, y + 1, w - 2, h - 2, activated_color(c));
    scheme1_up_frame_draw(x, y, w, h, c);
}

fn scheme1_down_box_draw(x: i32, y: i32, w: i32, h: i32, c: Color) {
    draw_rect_fill(x + 1, y + 1, w - 2, h - 2, activated_color(c));
    scheme1_down_frame_draw(x, y, w, h, c);
}

fn scheme1_up_frame_draw(x: i32, y: i32, w: i32, h: i32, c: Color) {
    app::set_frame_color(c.darker());
    draw_xyline2(x + 1, y, x + w - 2, y);
    draw_xyline2(x + 1, y + h - 1, x + w - 2, y + h - 1);
    draw_xyline2(x, y + 1, x, y + h - 2);
    draw_xyline2(x + w - 1, y + 1, x + w - 1, y + h - 2);

    app::set_frame_color(Color::color_average(c, c.darker(), 0.5));
    draw_xyline2(x + 2, y + h - 3, x + w - 2, y + h - 3);
    draw_xyline2(x + 1, y + h - 2, x + w - 2, y + h - 2);
    draw_xyline2(x + w - 3, y + 3, x + w - 3, y + h - 4);
    draw_xyline2(x + w - 2, y + 2, x + w - 2, y + h - 4);

    app::set_frame_color(Color::color_average(c, c.lighter(), 0.45));
    draw_xyline2(x + 1, y + 1, x + w - 2, y + 1);
    draw_xyline2(x + 1, y + 2, x + w - 3, y + 2);
    draw_xyline2(x + 1, y + 2, x + 1, y + h - 3);
    draw_xyline2(x + 2, y + 2, x + 2, y + h - 4);
}

fn scheme1_down_frame_draw(x: i32, y: i32, w: i32, h: i32, c: Color) {
    app::set_frame_color(Color::color_average(c, c.darker(), 0.15));
    draw_xyline2(x + 1, y, x + w - 2, y);
    draw_xyline2(x, y + 1, x + w - 1, y + 1);
    draw_xyline2(x, y + h - 2, x + w - 1, y + h - 2);
    draw_xyline2(x + 1, y + h - 1, x + w - 2, y + h - 1);
    draw_xyline2(x, y + 2, x, y + h - 3);
    draw_xyline2(x + 1, y + 2, x + 1, y + h - 3);
    draw_xyline2(x + w - 1, y + 2, x + w - 1, y + h - 3);
    draw_xyline2(x + w - 2, y + 2, x + w - 2, y + h - 3);

    app::set_frame_color(Color::color_average(c, c.darker(), 0.5));
    draw_xyline2(x + 2, y + 2, x + w - 3, y + 2);
    draw_xyline2(x + 2, y + 3, x + 2, y + h - 3);
}

fn scheme1_thin_up_box_draw(x: i32, y: i32, w: i32, h: i32, c: Color) {
    draw_rect_fill(x + 1, y + 1, w - 2, h - 2, activated_color(c));
    scheme1_thin_up_frame_draw(x, y, w, h, c);
}

fn scheme1_thin_down_box_draw(x: i32, y: i32, w: i32, h: i32, c: Color) {
    draw_rect_fill(x + 1, y + 1, w - 2, h - 2, activated_color(c));
    scheme1_thin_down_frame_draw(x, y, w, h, c);
}

fn scheme1_thin_up_frame_draw(x: i32, y: i32, w: i32, h: i32, c: Color) {
    app::set_frame_color(c.lighter());
    draw_xyline2(x + 1, y, x + w - 2, y);
    draw_xyline2(x, y + 1, x, y + h - 2);

    app::set_frame_color(Color::color_average(c, c.darker(), 0.5));
    draw_xyline2(x + 1, y + h - 1, x + w - 2, y + h - 1);
    draw_xyline2(x + w - 1, y + 1, x + w - 1, y + h - 2);
}

fn scheme1_thin_down_frame_draw(x: i32, y: i32, w: i32, h: i32, c: Color) {
    app::set_frame_color(c.darker());
    draw_xyline2(x + 1, y, x + w - 2, y);
    draw_xyline2(x, y + 1, x, y + h - 2);

    app::set_frame_color(Color::color_average(c, c.darker(), 0.5));
    draw_xyline2(x + 1, y + h - 1, x + w - 2, y + h - 1);
    draw_xyline2(x + w - 1, y + 1, x + w - 1, y + h - 2);
}

fn scheme2_up_box_draw(x: i32, y: i32, w: i32, h: i32, c: Color) {
    draw_rect_fill(x + 1, y + 1, w - 2, h - 2, activated_color(c));
    scheme2_up_frame_draw(x, y, w, h, c);
}

fn scheme2_down_box_draw(x: i32, y: i32, w: i32, h: i32, c: Color) {
    draw_rect_fill(x + 1, y + 1, w - 2, h - 2, activated_color(c));
    scheme2_down_frame_draw(x, y, w, h, c);
}

fn scheme2_up_frame_draw(x: i32, y: i32, w: i32, h: i32, c: Color) {
    app::set_frame_color(c.darker());
    draw_xyline2(x + 2, y, x + w - 3, y);
    draw_xyline2(x + 2, y + h - 1, x + w - 3, y + h - 1);
    draw_point(x + 1, y + 1);
    draw_point(x + 1, y + h - 2);
    draw_point(x + w - 2, y + 1);
    draw_point(x + w - 2, y + h - 2);
    draw_xyline2(x, y + 2, x, y + h - 3);
    draw_xyline2(x + w - 1, y + 2, x + w - 1, y + h - 3);

    app::set_frame_color(Color::color_average(c, c.lighter(), 0.25));
    draw_xyline2(x + 2, y + 1, x + w - 3, y + 1);
    draw_xyline2(x + 2, y + h - 2, x + w - 3, y + h - 2);

    app::set_frame_color(Color::color_average(c, c.lighter(), 0.45));
    draw_xyline2(x + 1, y + 2, x + w - 2, y + 2);
    draw_xyline2(x + 1, y + h - 3, x + w - 2, y + h - 3);

    app::set_frame_color(Color::color_average(c, c.lighter(), 0.65));
    draw_xyline2(x + 1, y + 3, x + w - 2, y + 3);
    draw_xyline2(x + 1, y + h - 4, x + w - 2, y + h - 4);

    app::set_frame_color(Color::color_average(c, c.lighter(), 0.85));
    draw_xyline2(x + 1, y + 4, x + w - 2, y + 4);
    draw_xyline2(x + 1, y + h - 5, x + w - 2, y + h - 5);
    draw_xyline2(x + 1, y + 5, x + 1, y + h - 6);
    draw_xyline2(x + w - 2, y + 5, x + w - 2, y + h - 6);
}

fn scheme2_down_frame_draw(x: i32, y: i32, w: i32, h: i32, c: Color) {
    app::set_frame_color(c.darker());
    draw_xyline2(x + 2, y, x + w - 3, y);
    draw_xyline2(x + 2, y + h - 1, x + w - 3, y + h - 1);
    draw_point(x + 1, y + 1);
    draw_point(x + 1, y + h - 2);
    draw_point(x + w - 2, y + 1);
    draw_point(x + w - 2, y + h - 2);
    draw_xyline2(x, y + 2, x, y + h - 3);
    draw_xyline2(x + w - 1, y + 2, x + w - 1, y + h - 3);

    app::set_frame_color(Color::color_average(c, c.darker(), 0.25));
    draw_xyline2(x + 2, y + 1, x + w - 3, y + 1);
    draw_xyline2(x + 2, y + h - 2, x + w - 3, y + h - 2);

    app::set_frame_color(Color::color_average(c, c.darker(), 0.45));
    draw_xyline2(x + 1, y + 2, x + w - 2, y + 2);
    draw_xyline2(x + 1, y + h - 3, x + w - 2, y + h - 3);

    app::set_frame_color(Color::color_average(c, c.darker(), 0.65));
    draw_xyline2(x + 1, y + 3, x + w - 2, y + 3);
    draw_xyline2(x + 1, y + h - 4, x + w - 2, y + h - 4);

    app::set_frame_color(Color::color_average(c, c.darker(), 0.85));
    draw_xyline2(x + 1, y + 4, x + w - 2, y + 4);
    draw_xyline2(x + 1, y + h - 5, x + w - 2, y + h - 5);
    draw_xyline2(x + 1, y + 5, x + 1, y + h - 6);
    draw_xyline2(x + w - 2, y + 5, x + w - 2, y + h - 6);
}

fn scheme2_thin_up_box_draw(x: i32, y: i32, w: i32, h: i32, c: Color) {
    draw_rect_fill(x + 1, y + 1, w - 2, h - 2, activated_color(c));
    scheme2_thin_up_frame_draw(x, y, w, h, c);
}

fn scheme2_thin_down_box_draw(x: i32, y: i32, w: i32, h: i32, c: Color) {
    draw_rect_fill(x + 1, y + 1, w - 2, h - 2, activated_color(c));
    scheme2_thin_down_frame_draw(x, y, w, h, c);
}

fn scheme2_thin_up_frame_draw(x: i32, y: i32, w: i32, h: i32, c: Color) {
    app::set_frame_color(Color::color_average(c, c.darker(), 0.45));
    draw_xyline2(x + 2, y, x + w - 3, y);
    draw_xyline2(x + 2, y + h - 1, x + w - 3, y + h - 1);
    draw_point(x + 1, y + 1);
    draw_point(x + 1, y + h - 2);
    draw_point(x + w - 2, y + 1);
    draw_point(x + w - 2, y + h - 2);
    draw_xyline2(x, y + 2, x, y + h - 3);
    draw_xyline2(x + w - 1, y + 2, x + w - 1, y + h - 3);

    app::set_frame_color(Color::color_average(c, c.lighter(), 0.65));
    draw_xyline2(x + 2, y + 1, x + w - 3, y + 1);
    draw_xyline2(x + 2, y + h - 2, x + w - 3, y + h - 2);

    app::set_frame_color(Color::color_average(c, c.lighter(), 0.85));
    draw_xyline2(x + 1, y + 2, x + w - 2, y + 2);
    draw_xyline2(x + 1, y + h - 3, x + w - 2, y + h - 3);
    draw_xyline2(x + 1, y + 3, x + 1, y + h - 3);
    draw_xyline2(x + w - 2, y + 3, x + w - 2, y + h - 3);
}

fn scheme2_thin_down_frame_draw(x: i32, y: i32, w: i32, h: i32, c: Color) {
    app::set_frame_color(Color::color_average(c, c.darker(), 0.45));
    draw_xyline2(x + 2, y, x + w - 3, y);
    draw_xyline2(x + 2, y + h - 1, x + w - 3, y + h - 1);
    draw_point(x + 1, y + 1);
    draw_point(x + 1, y + h - 2);
    draw_point(x + w - 2, y + 1);
    draw_point(x + w - 2, y + h - 2);
    draw_xyline2(x, y + 2, x, y + h - 3);
    draw_xyline2(x + w - 1, y + 2, x + w - 1, y + h - 3);

    app::set_frame_color(Color::color_average(c, c.darker(), 0.65));
    draw_xyline2(x + 2, y + 1, x + w - 3, y + 1);
    draw_xyline2(x + 2, y + h - 2, x + w - 3, y + h - 2);

    app::set_frame_color(Color::color_average(c, c.darker(), 0.85));
    draw_xyline2(x + 1, y + 2, x + w - 2, y + 2);
    draw_xyline2(x + 1, y + h - 3, x + w - 2, y + h - 3);
    draw_xyline2(x + 1, y + 3, x + 1, y + h - 3);
    draw_xyline2(x + w - 2, y + 3, x + w - 2, y + h - 3);
}

pub(crate) fn use_fleet_scheme1() {
    use fltk::enums::FrameType::*;
    app::reload_scheme().ok();
    app::set_scheme(app::Scheme::Gtk);
    app::set_frame_type_cb(UpBox, scheme1_up_box_draw, 1, 1, 2, 2);
    app::set_frame_type_cb(DownBox, scheme1_down_box_draw, 1, 1, 2, 2);
    app::set_frame_type_cb(UpFrame, scheme1_up_frame_draw, 1, 1, 2, 2);
    app::set_frame_type_cb(DownFrame, scheme1_down_frame_draw, 1, 1, 2, 2);
    app::set_frame_type_cb(ThinUpBox, scheme1_thin_up_box_draw, 1, 1, 2, 2);
    app::set_frame_type_cb(ThinDownBox, scheme1_thin_down_box_draw, 1, 1, 2, 2);
    app::set_frame_type_cb(ThinUpFrame, scheme1_thin_up_frame_draw, 1, 1, 2, 2);
    app::set_frame_type_cb(ThinDownFrame, scheme1_thin_down_frame_draw, 1, 1, 2, 2);
}

pub(crate) fn use_fleet_scheme2() {
    use fltk::enums::FrameType::*;
    app::reload_scheme().ok();
    app::set_scheme(app::Scheme::Gtk);
    app::set_frame_type_cb(UpBox, scheme2_up_box_draw, 1, 1, 2, 2);
    app::set_frame_type_cb(DownBox, scheme2_down_box_draw, 1, 1, 2, 2);
    app::set_frame_type_cb(UpFrame, scheme2_up_frame_draw, 1, 1, 2, 2);
    app::set_frame_type_cb(DownFrame, scheme2_down_frame_draw, 1, 1, 2, 2);
    app::set_frame_type_cb(ThinUpBox, scheme2_thin_up_box_draw, 1, 1, 2, 2);
    app::set_frame_type_cb(ThinDownBox, scheme2_thin_down_box_draw, 1, 1, 2, 2);
    app::set_frame_type_cb(ThinUpFrame, scheme2_thin_up_frame_draw, 1, 1, 2, 2);
    app::set_frame_type_cb(ThinDownFrame, scheme2_thin_down_frame_draw, 1, 1, 2, 2);
}
