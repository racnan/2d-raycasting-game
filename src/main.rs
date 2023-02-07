pub mod game;
use game::{model, update};

fn main() {
    nannou::app(model).update(update).run();
}
