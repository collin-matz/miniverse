use std::cell::{Ref, RefCell};
use glam::DVec2;

use crate::{
    model::{
        barnes_hut_tree::BHTree, body::{Body, functions}
    },
    controller::controller_body::ControllerBody
};

/// Structure for managing the Controller.
pub struct Controller {
    controller_bodies: RefCell<Vec<ControllerBody>>,
    screen_dimensions: (DVec2, DVec2),
    dt: f64,
    theta: f64
}

impl Controller {
    /// Creates a new Controller instance with an empty Bodies vector.
    pub fn new(screen_dimensions: (DVec2, DVec2), dt: f64, theta: f64) -> Self {
        Controller { controller_bodies: RefCell::new(vec![]), screen_dimensions, dt, theta }
    }

    /// Adds a new Body to the Controller's list of Bodies.
    pub fn add_body(&mut self, controller_body: ControllerBody) {
        self.controller_bodies.borrow_mut().push(controller_body);
    }

    pub fn add_all_bodies(&mut self, controller_body_list: Vec<ControllerBody>) {
        for body in controller_body_list {
            self.add_body(body);
        }
    }

    /// Returns a ViewBody list for rendering to the screen.
    pub fn update_bodies_and_get_positions(&self) -> Ref<Vec<ControllerBody>> {
        self.step();
        self.controller_bodies.borrow()
    }

    /// Steps the model by the specified dt.
    /// To do this, the Controller constructs a new BHTree index over the current
    /// Body list it contains. It then calculates the forces from this tree and applies
    /// those forces to each Body in the list.
    fn step(&self) {

        // Update each Body's position.
        for controller_body in self.controller_bodies.borrow_mut().iter_mut() {
            functions::update_position(&mut controller_body.body, self.dt);
        }

        // Build tree index over the Body list. 
        let mut bodies: Vec<Body> = vec![];

        for body in self.controller_bodies.borrow().iter() {
            bodies.push(body.body);
        }

        let mut tree = BHTree::<Body>::new(&bodies, self.screen_dimensions);

        // Calculate forces from this tree.
        let forces = tree.calculate_forces(self.dt, self.theta);

        // Update each Body's Velocity and Acceleration.
        for (i, body) in self.controller_bodies.borrow_mut().iter_mut().enumerate() {
            functions::apply_force_to_body(&mut body.body, forces[i], self.dt);
        };
    }
}