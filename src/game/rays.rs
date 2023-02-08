use nannou::{geom::Vec2, Draw};

use crate::game::{lines, utils};
pub struct Rays {
    rays: lines::Lines,
    number_of_rays: usize,
    centre: Vec2,
}

impl Rays {
    pub fn new(number_of_rays: usize) -> Self {
        Self {
            rays: lines::Lines::new_random(number_of_rays),
            centre: Vec2::new(0.0, 0.0),
            number_of_rays,
        }
    }

    pub fn update_rays(&mut self, centre: utils::MouseXY) {
        self.centre = Vec2::new(centre.0, centre.1);
        self.rays = lines::Lines::new(self.get_ray_points());
    }

    pub fn draw(&self, draw: &Draw) {
        self.rays.draw(draw)
    }

    fn get_ray_points(&self) -> Vec<utils::LineVec2s> {
        let angle_between_rays = 360 / self.number_of_rays;
        let mut ray_points = Vec::with_capacity(self.number_of_rays);

        let mut angle = 0;
        loop {
            let unit_vector = utils::get_unit_vector_from_angle(angle as f32);
            let end_position = self.centre + unit_vector * 50.0;
            ray_points.push((self.centre, end_position));
            angle += angle_between_rays;

            if angle >= 360 {
                break;
            }
        }
        ray_points
    }
}
