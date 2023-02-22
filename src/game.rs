use nannou::prelude::*;

mod ball;
mod consts;
mod lines;
mod rays;
mod types;
mod utils;

pub struct Model {
    ball: ball::Ball,
    walls: lines::Lines,
    rays: rays::Rays,
}

pub fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(consts::WINDOW_WIDTH, consts::WINDOW_HEIGHT)
        .view(view)
        .build()
        .unwrap();

    let ball = ball::Ball::new(consts::BALL_RAIDUS);
    let walls = lines::Lines::new_random(8, consts::WALL_WEIGHT);
    let rays = rays::Rays::new(60);
    Model { ball, walls, rays }
}

pub fn update(app: &App, model: &mut Model, _update: Update) {
    let mouse_xy = (app.mouse.x, app.mouse.y);
    model.ball.update_position(mouse_xy);
    model.rays.update_rays(mouse_xy, &model.walls);
}

pub fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(utils::get_window_bg_color());

    model.walls.draw(&draw);
    model.ball.draw(&draw);
    model.rays.draw(&draw);

    draw.to_frame(app, &frame).unwrap();
}
