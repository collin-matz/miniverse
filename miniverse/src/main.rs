mod view;
mod controller;
mod model;

use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new("My Window", WIDTH, HEIGHT, WindowOptions::default()).unwrap();

    // mainloop(&mut window, &mut buffer)
}
