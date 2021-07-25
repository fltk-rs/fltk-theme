use super::*;

fn rect(x: i32, y: i32, w: i32, h: i32, c: Color) {
    draw_rect_fill(x, y, w, h, c);
}

fn rectf(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(c));
    draw_rectf(x, y, w, h);
}

fn up_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    rect(x, y, w, h, Color::color_average(Color::White, c, 0.2));
}

fn up_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    up_frame(x, y, w, h, c);
}

fn default_up_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    rectf(
        x,
        y,
        w,
        h,
        Color::color_average(Color::Black, Color::White, 0.3),
    );
}

fn down_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    rect(
        x - 2,
        y - 2,
        w + 4,
        h + 4,
        Color::color_average(Color::Black, Color::White, 0.3),
    );
    rectf(x, y, w, h, c);
}

fn down_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    down_frame(x, y, w, h, c);
}

fn border_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    rectf(x, y, w, h, c);
    rect(x, y, w, h, Color::color_average(Color::White, c, 0.2));
}

fn round_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    rectf(x, y, w, h, c);
    set_draw_color(Color::from_rgb(0x00, 0x00, 0x00));
    draw_arc(x - 1, y - 1, w + 2, h + 2, 0.0, 360.0);
}

fn hover_up_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    rect(
        x - 2,
        y - 2,
        w + 4,
        h + 4,
        Color::color_average(Color::Black, Color::White, 0.2),
    );
}

fn hover_up_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    hover_up_frame(x - 2, y - 2, w + 4, h + 4, c);
    rect(
        x + 2,
        y + 2,
        w - 4,
        h - 4,
        Color::color_average(Color::Black, Color::White, 0.3),
    );
}

fn depressed_down_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    rect(
        x - 2,
        y - 2,
        w + 4,
        h + 4,
        Color::color_average(Color::Black, Color::White, 0.3),
    );
    rectf(x, y, w, h, c);
}

fn depressed_down_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    depressed_down_frame(x, y, w, h, c);
    set_draw_color(Color::color_average(Color::Black, Color::White, 0.2));
    draw_rectf(x, y, w, h);
}

fn use_fluent_scheme() {
    use fltk::enums::FrameType::*;
    app::set_scheme(app::Scheme::Base);
    app::set_frame_type_cb(UpBox, up_box, 1, 1, 2, 2);
    app::set_frame_type_cb(DownBox, down_box, 1, 1, 2, 2);
    app::set_frame_type_cb(ThinUpBox, up_box, 1, 1, 2, 2);
    app::set_frame_type_cb(ThinDownBox, down_box, 1, 1, 2, 2);
    app::set_frame_type_cb(UpFrame, up_frame, 1, 1, 2, 2);
    app::set_frame_type_cb(DownFrame, down_frame, 1, 1, 2, 2);
    app::set_frame_type_cb(RoundUpBox, round_box, 1, 1, 2, 2);
    app::set_frame_type_cb(RoundDownBox, round_box, 1, 1, 2, 2);
    app::set_frame_type_cb(BorderBox, border_box, 1, 1, 2, 2);
    app::set_frame_type2(OS_BUTTON_UP_FRAME, UpFrame);
    app::set_frame_type_cb(OS_DEFAULT_BUTTON_UP_BOX, default_up_box, 1, 1, 2, 2);
    app::set_frame_type2(OS_BUTTON_UP_BOX, UpBox);
    app::set_frame_type2(OS_CHECK_DOWN_BOX, DownBox);
    app::set_frame_type2(OS_CHECK_DOWN_FRAME, DownFrame);
    app::set_frame_type_cb(OS_HOVERED_UP_FRAME, hover_up_frame, 1, 1, 2, 2);
    app::set_frame_type_cb(OS_HOVERED_UP_BOX, hover_up_box, 1, 1, 2, 2);
    app::set_frame_type2(OS_RADIO_ROUND_DOWN_BOX, RoundDownBox);
    app::set_frame_type_cb(OS_DEPRESSED_DOWN_FRAME, depressed_down_frame, 1, 1, 2, 2);
    app::set_frame_type_cb(OS_DEPRESSED_DOWN_BOX, depressed_down_box, 1, 1, 2, 2);
    app::set_frame_type2(OS_DEFAULT_DEPRESSED_DOWN_BOX, OS_DEPRESSED_DOWN_BOX);
    app::set_frame_type2(OS_DEFAULT_HOVERED_UP_BOX, OS_HOVERED_UP_BOX);
    app::set_frame_type2(OS_INPUT_THIN_DOWN_FRAME, DownFrame);
    app::set_frame_type2(OS_INPUT_THIN_DOWN_BOX, DownBox);
}

fn use_fluent_colors() {
    app::background(0xF0, 0xF0, 0xF0);
    app::background2(0xFF, 0xFF, 0xFF);
    app::foreground(0x00, 0x00, 0x00);
    app::set_color(Color::Inactive, 0x6F, 0x6F, 0x6F);
    app::set_color(Color::Selection, 0x33, 0x99, 0xFF);
    app::set_color(Color::Free, 0xFF, 0xFF, 0xFF);
    Tooltip::set_color(Color::from_rgb(0xFF, 0xFF, 0xFF));
    Tooltip::set_text_color(Color::ForeGround);
}

pub(crate) fn use_fluent_theme() {
    use_fluent_scheme();
    use_fluent_colors();
    use_native_settings();
}
