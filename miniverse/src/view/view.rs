use minifb::{Key, Window, WindowOptions, MouseButton, MouseMode};
use crate::{
    controller::{controller_body::{ControllerBody, ControllerBodyBuilder}, controller::Controller}, 
    model::barnes_hut_tree::HasPositionAndMassTrait
};


/// Structure for managing information regarding the ViewLoop
pub struct ViewLoop<'v> {
    controller: &'v mut Controller,             // An immutable reference to the controller.
    main_buffer: Vec<u32>,
    pixel_queue_buffer: Vec<Option<u32>>,       // Stores the pixel to be rendered on the next pass.
    window: Window,
    width: usize,
    height: usize,
    scale: f64,                                 // Scales the pixel values to work with Universe distances.
    universe_background_color: u32,
    planet_default_color: u32
}

impl<'v> ViewLoop<'v> {
    /// Creates a new ViewLoop instance.
    pub fn new(controller: &'v mut Controller, width: usize, height: usize, scale: f64, app_title: &str) -> Self {
        ViewLoop { 
            controller,
            main_buffer: vec![0; width * height],
            pixel_queue_buffer: vec![None; width * height],
            window: Window::new(&app_title, width, height, WindowOptions::default()).expect("Failed to update buffer"), 
            width, 
            height, 
            scale,
            universe_background_color: 0x00000000,
            planet_default_color: 0xFFFFFFFF
        }
    }

    /// Starts the main ViewLoop.
    pub fn start(&mut self) {
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {

            if self.window.get_mouse_down(MouseButton::Left) {
                if let Some((x, y)) = self.window.get_mouse_pos(MouseMode::Clamp) {

                    let position = self.pixel_to_world(x, y);

                    println!("{:?}", position);

                    let cb = ControllerBodyBuilder::new()
                        .mass(1e30)
                        .position_from_point(position.0, position.1)
                        .build();

                    self.controller.add_body(cb);
                }

                
            }

            // Clear the pixel_queue_buffer
            self.clear_pixel_queue_buffer();

            // Get the next set of Body positions for rendering.
            let controller_body_list: Vec<ControllerBody> = self.controller.update_bodies_and_get_positions().iter().cloned().collect();

            for controller_body in controller_body_list.iter() {

                // Convert the physical position of the body to a pixel position.
                let position = self.world_to_pixel(
                    controller_body.body.position().x, controller_body.body.position().y
                );


                if let Some((px, py)) = position {
                    // Color in the circle that the Body takes up
                    self.draw_filled_circle(px, py, controller_body.radius, controller_body.color);
                }
            }

            // Every frame, go over the entire buffer. If there is something in the pixel queue,
            // render that instead of what's in the main buffer.
            for i in 0..self.main_buffer.len() {

                match self.pixel_queue_buffer[i] {
                    Some(color) => {
                        self.main_buffer[i] = color;
                    },
                    None => {
                        self.main_buffer[i] = self.universe_background_color;
                    }
                }
            }

            // Update the window with the contents of the main buffer.
            self.window.update_with_buffer(&self.main_buffer, self.width, self.height).unwrap();
        }
    }

    fn clear_pixel_queue_buffer(&mut self) {
        self.pixel_queue_buffer = vec![None; self.width * self.height];
    }

    /// Convert a world pixel to a screen pixel.
    fn world_to_pixel(&self, x: f64, y: f64) -> Option<(i32, i32)> {
        let px = (x / self.scale + self.width as f64 / 2.0) as isize;
        let py = (y / self.scale + self.height as f64 / 2.0) as isize;
        

        // Perform a bounds check
        if px >= 0 && px < self.width as isize && py >= 0 && py < self.height as isize {
            Some((px as i32, py as i32))
        } else {
            None
        }
    }

    /// Convert a screen pixel to a world pixel.
    pub fn pixel_to_world(&self, px: f32, py: f32) -> (f64, f64) {
        let world_x = (px as f64 - self.width as f64 / 2.0) * self.scale;
        let world_y = (py as f64 - self.height as f64 / 2.0) * self.scale;
        (world_x as f64, world_y as f64)
    }

    fn draw_filled_circle(&mut self, cx: i32, cy: i32, radius: u32, color: u32) {
        let radius = radius as i32;
        let r2 = radius * radius;

        for dy in -radius..=radius {
            for dx in -radius..=radius {
                if dx * dx + dy * dy <= r2 {
                    let x = cx + dx;
                    let y = cy + dy;

                    if x >= 0 && x < self.width as i32 &&
                    y >= 0 && y < self.height as i32 {
                        let index = y as usize * self.width + x as usize;
                        self.pixel_queue_buffer[index] = Some(color);
                    }
                }
            }
        }
    }
}
