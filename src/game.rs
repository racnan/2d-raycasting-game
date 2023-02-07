use nannou::prelude::*;

mod ball;
mod consts;
mod lines;
mod utils;

pub struct Model {
    ball: ball::Ball,
    lines: lines::Lines,
}

pub fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(consts::WINDOW_WIDTH, consts::WINDOW_HEIGHT)
        .view(view)
        .build()
        .unwrap();

    let ball = ball::Ball::new(consts::BALL_RAIDUS);
    let lines = lines::Lines::new(2);
    Model { ball, lines }
}

pub fn update(_app: &App, _model: &mut Model, _update: Update) {}

pub fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(utils::get_window_bg_color());

    model.lines.draw(&draw);

    let mouse_xy = (app.mouse.x, app.mouse.y);
    model.ball.draw(&draw, mouse_xy);

    draw.to_frame(app, &frame).unwrap();
}
