use nannou::{geom::Vec2, Draw};

use crate::game::{consts, lines, types, utils};

pub struct Rays {
    rays: lines::Lines,
    number_of_rays: usize,
    centre: Vec2,
}

impl Rays {
    pub fn new(number_of_rays: usize) -> Self {
        Self {
            rays: lines::Lines::new_random(number_of_rays, consts::RAY_WEIGHT),
            centre: Vec2::new(0.0, 0.0),
            number_of_rays,
        }
    }

    pub fn update_rays(&mut self, centre: types::MouseXY, walls: &lines::Lines) {
        self.centre = Vec2::new(centre.0, centre.1);
        self.rays = lines::Lines::new(self.get_ray_points(walls), consts::RAY_WEIGHT);
    }

    pub fn draw(&self, draw: &Draw) {
        self.rays.draw(draw)
    }

    fn get_ray_points(&self, walls: &lines::Lines) -> Vec<types::LineVec2s> {
        let angle_between_rays = 360 / self.number_of_rays;
        let mut ray_points = Vec::with_capacity(self.number_of_rays);

        let mut angle = 0;
        loop {
            let unit_vector = utils::get_unit_vector_from_angle(angle as f32);
            let ray_vector = utils::get_ray_vector(unit_vector, self.centre);

            if let Some(intersection) = get_ray_intersection(walls, (self.centre, ray_vector)) {
                ray_points.push((self.centre, intersection));
            }

            angle += angle_between_rays;
            if angle >= 360 {
                break;
            }
        }
        ray_points
    }
}

fn get_ray_intersection(walls: &lines::Lines, ray: types::LineVec2s) -> Option<Vec2> {
    let centre = ray.0;
    let mut intersections = Vec::new();
    for wall in walls.get_lines() {
        if let Some(intersection) = get_line_vector_intersection(wall, ray) {
            intersections.push(intersection);
        }
    }

    if let Some((mut closest_intersection, mut shortest_distance)) =
        utils::get_first_intersection_and_distance(&intersections, centre)
    {
        for intersection in intersections {
            if centre.distance(intersection) < shortest_distance {
                shortest_distance = centre.distance(intersection);
                closest_intersection = intersection;
            }
        }
        return Some(closest_intersection);
    }

    None
}

fn get_line_vector_intersection(wall: &lines::Line, ray: types::LineVec2s) -> Option<Vec2> {
    let wall_start_end_vecs = wall.get_start_and_end_vector();
    let (x1, y1, x2, y2) = utils::unwrap_line_vecs_to_coordinates(wall_start_end_vecs);
    let (x3, y3, x4, y4) = utils::unwrap_line_vecs_to_coordinates(ray);

    let denominator = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
    if denominator == 0.0 {
        return None;
    }

    let t = (((x1 - x3) * (y3 - y4)) - ((y1 - y3) * (x3 - x4))) / denominator;
    let u = (((x1 - x3) * (y1 - y2)) - ((y1 - y3) * (x1 - x2))) / denominator;

    if (0.0..=1.0).contains(&t) && u >= 0.0 {
        let intersection_x = x1 + t * (x2 - x1);
        let intersection_y = y1 + t * (y2 - y1);
        let intersection_vector = Vec2::new(intersection_x, intersection_y);
        return Some(intersection_vector);
    }
    None
}
