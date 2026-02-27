mod model;

use crate::model::{
    barnes_hut_tree::{BHTree, HasPositionAndMassTrait}, body::{Body2DTrait, BodyBuilder, functions}
};

use glam::DVec2;
use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 1000;
const HEIGHT: usize = 1000;
const SCALE: f64 = 7.48e8;

const half_w: f64 = WIDTH as f64 * SCALE * 0.5;
const half_h: f64 = HEIGHT as f64 * SCALE * 0.5;

const dimensions: (DVec2, DVec2) = (DVec2::new(-half_w, -half_h), DVec2::new( half_w,  half_h));

fn world_to_pixel(x: f64, y: f64, width: usize, height: usize, scale: f64) -> Option<(usize, usize)> {
    let px = (x / scale + width as f64 / 2.0) as isize;
    let py = (y / scale + height as f64 / 2.0) as isize;
    

    // Perform a bounds check
    if px >= 0 && px < width as isize && py >= 0 && py < height as isize {
        Some((px as usize, py as usize))
    } else {
        None
    }
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new("Minverse", WIDTH, HEIGHT, WindowOptions::default()).expect("Failed to update buffer");

    // Create bodies.
    let mut bodies = vec![
        BodyBuilder::new().position_from_point(0.0, 0.0).mass(1.989e30).velocity_from_point(0.0, 0.0).build(),
        BodyBuilder::new().position_from_point(1.496e11, 0.0).mass(5.972e24).velocity_from_point(0.0, 29780.0).build(),       
    ];

    let dt: f64 = 10000.0;
    let theta = 0.0;

    // Build tree.
    let mut tree = BHTree::new(&bodies, dimensions);

    // Get forces
    let forces = tree.calculate_forces(dt, theta);

    for (i, body) in bodies.iter_mut().enumerate() {
        let acc = forces[i] / body.mass();
        body.set_acceleration(acc); 
    }

    println!("Earth accel: {}", bodies[1].acceleration().length());
    println!("Sun accel: {}", bodies[0].acceleration().length());

    while window.is_open() && !window.is_key_down(Key::Escape) {

        // Fill buffer with a color (0x00RRGGBB format)
        for pixel in buffer.iter_mut() {
            *pixel = 0x00000000; // black
        }

        // fill the Body positions.
        for (i, body) in bodies.iter_mut().enumerate() {
            let new_pos = body.position() + body.velocity() * dt + 0.5 * body.acceleration() * dt * dt;
            body.set_position(new_pos);
        }

        // Build tree.
        let mut tree = BHTree::new(&bodies, dimensions);

        // Get forces
        let forces = tree.calculate_forces(dt, theta);

        // fill the Body positions.
        for (i, body) in bodies.iter_mut().enumerate() {
            let old_acc = body.acceleration();
            let new_acc = forces[i] / body.mass();

            let new_vel =
                body.velocity()
                + 0.5 * (old_acc + new_acc) * dt;

            body.set_velocity(new_vel);
            body.set_acceleration(new_acc);
        }

        for (i, body) in bodies.iter_mut().enumerate() {
            if let Some((px, py)) = world_to_pixel(body.position().x, body.position().y, WIDTH, HEIGHT, SCALE) {
                buffer[py * WIDTH + px] = 0x00FFFFFF;
            }
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}