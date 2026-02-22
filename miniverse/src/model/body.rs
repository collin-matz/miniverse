use glam::DVec2;
use crate::model::{
    constants::Constants,
    types::{Body2D}
};

#[derive(Copy, Clone)]
pub struct Body {
    mass: f64,
    position: DVec2,
    velocity: DVec2,
    acceleration: DVec2
}

impl Body2D for Body {
    fn mass(&self) -> f64 {
        self.mass
    }

    fn position(&self) -> DVec2 {
        self.position
    }

    fn distance<B: Body2D>(&self, other: &B) -> f64 {
        (
            (self.position.x - other.position().x).powf(2.0) + 
            (self.position.y - other.position().y).powf(2.0)
        ).sqrt()
    }

    /// Apply a force exerted from Body B onto Body A using Verlet integration.
    fn apply_force_from<B: Body2D>(&mut self, other: &B) {
        // Calculate the force from gravity.
        let f_g = Constants::G * ((self.mass * other.mass()) / self.distance(other));

        // Update this Body's position.
        self.position = self.position + self.velocity * Constants::DT + 0.5 * self.acceleration;

        // Calculate the updated acceleration (a = F/m).
        let old_acc = self.acceleration;
        let new_acc = f_g / self.mass;

        // 3. Update velocity using average of old and new accelerations.
        self.velocity = self.velocity + 0.5 * (old_acc + new_acc) * Constants::DT;
    }
}

pub struct BodyBuilder {
    mass: f64,
    position: DVec2,
    velocity: DVec2,
    acceleration: DVec2
}

impl BodyBuilder {
    pub fn mass(&mut self, mass: f64) -> &mut Self {
        self.mass = mass;
        self
    }

    pub fn position(&mut self, position: DVec2) -> &mut Self {
        self.position = position;
        self
    }

    pub fn velocity(&mut self, velocity: DVec2) -> &mut Self {
        self.velocity = velocity;
        self
    }

    pub fn acceleration(&mut self, acceleration: DVec2) -> &mut Self {
        self.acceleration = acceleration;
        self
    }

    pub fn new() -> Self {
        BodyBuilder {
            mass: 0.0,
            position: DVec2::ZERO,
            velocity: DVec2::ZERO,
            acceleration: DVec2::ZERO
        }
    }

    pub fn build(&mut self) -> Body {
        Body {
            mass: self.mass,
            position: self.position,
            velocity: self.velocity,
            acceleration: self.acceleration
        }
    }
}