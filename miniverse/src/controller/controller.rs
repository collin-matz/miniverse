use std::cell::RefCell;
use glam::DVec2;

use crate::{
    model::{
        barnes_hut_tree::{BHTree, HasPositionAndMassTrait}, body::{Body, functions}
    },
    view::view::ViewBody
};

/// A wrapper for the Body struct.
/// This wrapper allows the controller to know more information about the
/// body without needing to inject this information into the model (things
/// like color and radius that are important for the View).
struct ControllerBody {
    pub body: Body,
    pub color: u32,
    pub radius: i32,
    pub z_index: u32
}

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
    pub fn add_body(&mut self, body: Body, color: u32, radius: i32, z_index: u32) {
        // Since we always push a Body to the back of the list, we will
        let controller_body = ControllerBody {
            body, color, radius, z_index
        };
        self.controller_bodies.borrow_mut().push(controller_body);
    }

    pub fn add_all_bodies(&mut self, body_list: Vec<Body>) {
        for body in body_list {
            self.add_body(body, 0xFFFFFFFF, 10, 1);
        }
    }

    /// Returns a ViewBody list for rendering to the screen.
    pub fn update_bodies_and_get_positions(&self) -> Vec<ViewBody> {
        self.step();
        self.create_view_body_list()
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

    /// Using the ControllerBody list, create a new list of
    /// ViewBodies to send to the View.
    fn create_view_body_list(&self) -> Vec<ViewBody> {
        let mut view_body_list: Vec<ViewBody> = vec![];

        for controller_body in self.controller_bodies.borrow().iter() {
            view_body_list.push(ViewBody {
                body_position: controller_body.body.position(),
                color: controller_body.color,
                radius: controller_body.radius,
                z_index: controller_body.z_index
            })
        }

        view_body_list
    }
}