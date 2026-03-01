use glam::DVec2;

mod controller;
mod model;
mod view;

use view::view::ViewLoop;
use controller::controller::Controller;
use model::body::BodyBuilder;



const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const HALF_W: f64 = WIDTH as f64 * SCALE * 0.5;
const HALF_H: f64 = HEIGHT as f64 * SCALE * 0.5;
const DIMENSIONS: (DVec2, DVec2) = (DVec2::new(-HALF_W, -HALF_H), DVec2::new( HALF_W,  HALF_H));

const SCALE: f64 = 7.48e8;
const DT: f64 = 10000.0;
const THETA: f64 = 0.0;

const TITLE: &str = "Miniverse";

fn main() {
    // Create bodies.
    let mut bodies = vec![
        BodyBuilder::new().position_from_point(0.0, 0.0).mass(1.989e30).velocity_from_point(0.0, 0.0).build(),
        BodyBuilder::new().position_from_point(1.496e11, 0.0).mass(5.972e24).velocity_from_point(0.0, 29780.0).build(),       
    ];

    let mut controller = Controller::new(DIMENSIONS, DT, THETA);
    controller.add_all_bodies(bodies);

    let mut viewLoop = ViewLoop::new(&mut controller, WIDTH, HEIGHT, SCALE, TITLE);
    viewLoop.start();
}