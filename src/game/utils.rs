use nannou::{color, geom::Vec2, math::deg_to_rad};

use crate::game::{consts, types};

pub fn get_first_intersection_and_distance(intersections: &[Vec2], centre: Vec2) -> Option<(Vec2,f32)> {
    intersections.first().map(|intersection|{
        let intersection = intersection.to_owned();
        (intersection.to_owned(),centre.distance(intersection))
    })
}

pub fn get_unit_vector_from_angle(angle: f32) -> Vec2 {
    let angle = deg_to_rad(angle);
    let vector = Vec2::new(angle.cos(), angle.sin());
    vector.normalize()
}

pub fn get_ray_vector(unit_vec: Vec2, centre: Vec2) -> Vec2 {
    let x = centre.x + unit_vec.x;
    let y = centre.y + unit_vec.y;
    Vec2::new(x, y)
}

pub fn unwrap_line_vecs_to_coordinates(vecs: types::LineVec2s) -> (f32, f32, f32, f32) {
    let (start_x, start_y) = unwrap_vec_to_coordinates(vecs.0);
    let (end_x, end_y) = unwrap_vec_to_coordinates(vecs.1);
    (start_x, start_y, end_x, end_y)
}

pub fn unwrap_vec_to_coordinates(vec: Vec2) -> (f32, f32) {
    (vec.x, vec.y)
}

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
