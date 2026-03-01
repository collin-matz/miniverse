use glam::DVec2;
use crate::model::body::{Body, Body2DTrait, BodyBuilder};


/// A wrapper for the Body struct.
/// This wrapper allows the controller to know more information about the
/// body without needing to inject this information into the model (things
/// like color and radius that are important for the View).
#[derive(Clone)]
pub struct ControllerBody {
    pub body: Body,
    pub color: u32,
    pub radius: u32,
    pub z_index: u32
}

pub struct ControllerBodyBuilder {
    pub body: Body,
    pub color: u32,
    pub radius: u32,
    pub z_index: u32
}

impl ControllerBodyBuilder {
    /// Return a new ControllerBodyBuilder instance with default values.
    /// All values default to 0.0 for f64's and DVec2::ZERO for vectors.
    pub fn new() -> Self {
        ControllerBodyBuilder {
            body: BodyBuilder::new().build(),
            color: 0xFFFFFFFF,
            radius: 1,
            z_index: 0
        }
    }

    /// Set the mass of the Body from a f64.
    pub fn mass(&mut self, mass: f64) -> &mut Self {
        self.body.set_mass(mass);
        self
    }

    /// Set the position of the Body from a 2D vector.
    pub fn position_from_vector2D(&mut self, position: DVec2) -> &mut Self {
        self.body.set_position(position);
        self
    }

    /// Set the position of the Body from a 2D coordinate.
    pub fn position_from_point(&mut self, x: f64, y: f64) -> &mut Self {
        // Create a new DVec2 from the (x,y) point.
        self.body.set_position(DVec2::new(x, y));
        self
    }

    /// Set the velocity of the Body from a 2D vector.
    pub fn velocity_from_vector2D(&mut self, velocity: DVec2) -> &mut Self {
        self.body.set_velocity(velocity);
        self
    }

    /// Set the velocity of the Body from a 2D coordinate.
    pub fn velocity_from_point(&mut self, x_vel: f64, y_vel: f64) -> &mut Self {
        self.body.set_velocity(DVec2::new(x_vel, y_vel)); 
        self
    }

    /// Set the acceleration of the Body from a 2D vector.
    pub fn acceleration_from_vector2D(&mut self, acceleration: DVec2) -> &mut Self {
        self.body.set_acceleration(acceleration);
        self
    }

    /// Set the acceleration of the Body from a 2D vector.
    pub fn acceleration_from_point(&mut self, x_acc: f64, y_acc: f64) -> &mut Self {
        self.body.set_acceleration(DVec2::new(x_acc, y_acc));
        self
    }

    pub fn color(&mut self, color: u32) ->&mut Self {
        self.color = color;
        self
    }

    pub fn radius(&mut self, radius: u32) ->&mut Self {
        self.radius = radius;
        self
    }

    pub fn z_index(&mut self, z_index: u32) ->&mut Self {
        self.z_index = z_index;
        self
    }

    /// Build a Body instance using the values stored in the BodyBuilder up to this point.
    pub fn build(&mut self) -> ControllerBody {
        ControllerBody {
            body: self.body,
            color: self.color,
            radius: self.radius,
            z_index: self.z_index
        }
    }
}
