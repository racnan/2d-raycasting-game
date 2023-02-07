use nannou::prelude::*;

use crate::game::utils;

pub struct Ball {
    radius: f32,
    color: nannou::color::Rgb,
}

impl Ball {
    pub fn new(radius: f32) -> Self {
        Self {
            radius,
            color: utils::get_ball_color(),
        }
    }

    pub fn draw(&self, draw: &Draw, (x, y): utils::MouseXY) {
        draw.ellipse()
            .radius(self.radius)
            .color(self.color)
            .x(x)
            .y(y);
    }
}
