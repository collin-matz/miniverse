use glam::DVec2;

mod controller;
mod model;
mod view;
mod presets;

use view::view::ViewLoop;
use controller::{
    controller_body::ControllerBodyBuilder,
    controller::Controller
};


const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const HALF_W: f64 = WIDTH as f64 * SCALE * 0.5;
const HALF_H: f64 = HEIGHT as f64 * SCALE * 0.5;
const DIMENSIONS: (DVec2, DVec2) = (DVec2::new(-HALF_W, -HALF_H), DVec2::new( HALF_W,  HALF_H));

const SCALE: f64 = 7.48e9;
const DT: f64 = 60000.0;
const THETA: f64 = 0.5;

const TITLE: &str = "Miniverse";

fn main() {
    // Create bodies.
    // let mut bodies = vec![
    //     // Sun
    //     ControllerBodyBuilder::new()
    //         .position_from_point(0.0, 0.0)
    //         .mass(1.989e30)
    //         .velocity_from_point(0.0, 0.0)
    //         .color(0xFFFF00)
    //         .radius(10)
    //         .build(),
    //     // Earth
    //     ControllerBodyBuilder::new()
    //         .position_from_point(1.496e11, 0.0)
    //         .mass(5.972e24)
    //         .velocity_from_point(0.0, 29780.0)
    //         .color(0x0000FF)
    //         .radius(2)
    //         .build(),
    //     // Mars
    //     ControllerBodyBuilder::new()
    //         .position_from_point(2.279e11, 0.0)
    //         .mass(6.39e23)
    //         .velocity_from_point(0.0, 24100.0)
    //         .color(0xFF0000)
    //         .radius(2)
    //         .build(),
    // ];

    let mut bodies = vec![
        // Sun
        ControllerBodyBuilder::new()
            .position_from_point(0.0, 0.0)
            .mass(1.989e30)
            .velocity_from_point(0.0, 0.0)
            .color(0xFFFF00)
            .radius(6)
            .build(),
        // Mercury
        ControllerBodyBuilder::new()
            .position_from_point(5.791e10, 0.0)
            .mass(3.285e23)
            .velocity_from_point(0.0, 47400.0)
            .color(0xAAAAAA)
            .radius(1)
            .build(),
        // Venus
        ControllerBodyBuilder::new()
            .position_from_point(1.082e11, 0.0)
            .mass(4.867e24)
            .velocity_from_point(0.0, 35020.0)
            .color(0xFFCC44)
            .radius(2)
            .build(),
        // Earth
        ControllerBodyBuilder::new()
            .position_from_point(1.496e11, 0.0)
            .mass(5.972e24)
            .velocity_from_point(0.0, 29780.0)
            .color(0x0000FF)
            .radius(2)
            .build(),
        // Mars
        ControllerBodyBuilder::new()
            .position_from_point(2.279e11, 0.0)
            .mass(6.39e23)
            .velocity_from_point(0.0, 24100.0)
            .color(0xFF0000)
            .radius(2)
            .build(),
        // Jupiter
        ControllerBodyBuilder::new()
            .position_from_point(7.785e11, 0.0)
            .mass(1.898e27)
            .velocity_from_point(0.0, 13070.0)
            .color(0xFFAA66)
            .radius(5)
            .build(),
        // Saturn
        ControllerBodyBuilder::new()
            .position_from_point(1.432e12, 0.0)
            .mass(5.683e26)
            .velocity_from_point(0.0, 9690.0)
            .color(0xFFDD99)
            .radius(4)
            .build(),
        // Uranus
        ControllerBodyBuilder::new()
            .position_from_point(2.867e12, 0.0)
            .mass(8.681e25)
            .velocity_from_point(0.0, 6810.0)
            .color(0x99FFFF)
            .radius(3)
            .build(),
        // Neptune
        ControllerBodyBuilder::new()
            .position_from_point(4.515e12, 0.0)
            .mass(1.024e26)
            .velocity_from_point(0.0, 5430.0)
            .color(0x3333FF)
            .radius(3)
            .build(),
    ];

    let mut controller = Controller::new(DIMENSIONS, DT, THETA);
    controller.add_all_bodies(bodies);

    let mut viewLoop = ViewLoop::new(&mut controller, WIDTH, HEIGHT, SCALE, TITLE);
    viewLoop.start();
}