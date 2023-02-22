use nannou::{draw::Draw, geom::Vec2};

use crate::game::{types, utils};

pub struct Lines {
    lines: Vec<Line>,
}

impl Lines {
    pub fn new_random(number_of_lines: usize, line_weight: f32) -> Self {
        let (window_height_max, window_height_min) = utils::get_window_height_range();
        let (window_width_max, window_width_min) = utils::get_window_width_range();

        let mut lines = Vec::with_capacity(number_of_lines);
        for _ in 0..number_of_lines {
            let start_x = nannou::rand::random_range(window_width_min, window_width_max);
            let end_x = nannou::rand::random_range(window_width_min, window_width_max);

            let start_y = nannou::rand::random_range(window_height_min, window_height_max);
            let end_y = nannou::rand::random_range(window_height_min, window_height_max);

            let start = Vec2::new(start_x, start_y);
            let end = Vec2::new(end_x, end_y);

            let line = Line::new(start, end, line_weight);
            lines.push(line);
        }

        Self { lines }
    }

    pub fn new(coordinates: Vec<types::LineVec2s>, line_weight: f32) -> Self {
        let mut lines = Vec::with_capacity(coordinates.len());
        for (start, end) in coordinates {
            let line = Line::new(start, end, line_weight);
            lines.push(line);
        }
        Self { lines }
    }

    pub fn draw(&self, draw: &Draw) {
        for line in self.lines.iter() {
            line.draw(draw)
        }
    }

    pub fn get_lines(&self) -> &Vec<Line> {
        &self.lines
    }
}

#[derive(Debug)]
pub struct Line {
    start: Vec2,
    end: Vec2,
    weight: f32,
    color: nannou::color::Rgb,
}

impl Line {
    fn new(start: Vec2, end: Vec2, weight: f32) -> Self {
        Self {
            start,
            end,
            weight,
            color: utils::get_line_color(),
        }
    }

    fn draw(&self, draw: &Draw) {
        draw.line()
            .points(self.start, self.end)
            .color(self.color)
            .weight(self.weight);
    }

    pub fn get_start_and_end_vector(&self) -> types::LineVec2s {
        (self.start, self.end)
    }
}
