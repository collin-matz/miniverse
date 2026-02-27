use crate::model::{
    body::{Body2DTrait, BodyBuilder},
    barnes_hut_tree::{BHTree, HasPositionAndMassTrait}
};

use glam::DVec2;
use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 600;
const HEIGHT: usize = 600;

fn world_to_pixel(x: f64, y: f64, width: usize, height: usize, scale: f64) -> Option<(usize, usize)> {
    let px = (x * scale + width as f64 / 2.0) as isize;
    let py = (y * scale + height as f64 / 2.0) as isize;
    

    // Perform a bounds check
    if px >= 0 && px < width as isize && py >= 0 && py < height as isize {
        Some((px as usize, py as usize))
    } else {
        None
    }
}

fn main() {
    println!("Starting...");

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new("Minverse", WIDTH, HEIGHT, WindowOptions::default()).expect("Failed to update buffer");

    println!("Starting...");

    // Create bodies.
    let mut bodies = vec![
        BodyBuilder::new().position_from_point(10.0, 10.0).mass(1e14).build(),
        BodyBuilder::new().position_from_point(129.0, 10.0).mass(2e14).build(),
        BodyBuilder::new().position_from_point(192.0, 64.0).mass(3e14).build()
    ];

    println!("Made bodies...");

    while window.is_open() && !window.is_key_down(Key::Escape) {

        // // Build tree.
        // let tree = BHTree::new(
        //     &bodies, 
        //     (DVec2::new(0.0, WIDTH as f64), DVec2::new(0.0, HEIGHT as f64))
        // );

        // Fill buffer with a color (0x00RRGGBB format)
        for pixel in buffer.iter_mut() {
            *pixel = 0x00000000; // black
        }

        // // fill the Body positions as orange.
        // for body in &bodies {
        //     if let Some((px, py)) = world_to_pixel(body.position().x, body.position().y, WIDTH, HEIGHT, 1.0) {
        //         buffer[py * WIDTH + px] = 0x00FFFFFF;
        //     }
        // }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}