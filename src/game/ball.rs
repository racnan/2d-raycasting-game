use nannou::draw::Draw;

use crate::game::{types, utils};

pub struct Ball {
    radius: f32,
    position: (f32, f32),
    color: nannou::color::Rgb,
}

impl Ball {
    pub fn new(radius: f32) -> Self {
        Self {
            radius,
            position: (0.0, 0.0),
            color: utils::get_ball_color(),
        }
    }

    pub fn draw(&self, draw: &Draw) {
        draw.ellipse()
            .radius(self.radius)
            .color(self.color)
            .x(self.position.0)
            .y(self.position.1);
    }

    pub fn update_position(&mut self, position: types::MouseXY) {
        self.position = position;
    }
}
