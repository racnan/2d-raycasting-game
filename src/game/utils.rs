use nannou::color;

use crate::game::consts;

pub type MouseXY = (f32, f32);

pub fn get_window_height_range() -> (f32, f32) {
    let window_height = consts::WINDOW_HEIGHT as f32;
    (-(window_height / 2.0), window_height / 2.0)
}

pub fn get_window_width_range() -> (f32, f32) {
    let window_width = consts::WINDOW_WIDTH as f32;
    (-(window_width / 2.0), window_width / 2.0)
}
pub fn get_ball_color() -> color::Rgb {
    get_color_from_tuple(consts::BALL_COLOR)
}

pub fn get_line_color() -> color::Rgb {
    get_color_from_tuple(consts::LINE_COLOR)
}

pub fn get_window_bg_color() -> color::Rgb {
    get_color_from_tuple(consts::WINDOW_BG_COLOR)
}

fn get_color_from_tuple((r, g, b): (f32, f32, f32)) -> color::Rgb {
    color::rgb(r, g, b)
}
