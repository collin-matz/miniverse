use glam::DVec2;
use crate::model::{
    constants::Constants,
    barnes_hut_tree::HasPositionAndMassTrait,
};

/// A trait representing 2D Bodies.
pub trait Body2DTrait: Clone + HasPositionAndMassTrait {
    // Getters
    fn velocity(&self) -> DVec2;
    fn acceleration(&self) -> DVec2;

    // Setters
    fn set_mass(&mut self, mass: f64);
    fn set_position(&mut self, position: DVec2);
    fn set_velocity(&mut self, velocity: DVec2);
    fn set_acceleration(&mut self, acceleration: DVec2);
}

impl HasPositionAndMassTrait for Body {
    fn mass(&self) -> f64 { self.mass }
    fn position(&self) -> DVec2 { self.position }
}

#[derive(Copy, Clone)]
pub struct Body {
    mass: f64,
    position: DVec2,
    velocity: DVec2,
    acceleration: DVec2
}

impl Body2DTrait for Body {
    // Getters
    fn velocity(&self) -> DVec2 { self.velocity }
    fn acceleration(&self) -> DVec2 { self.acceleration }

    // Setters
    fn set_mass(&mut self, mass: f64) { self.mass = mass }
    fn set_position(&mut self, position: DVec2) { self.position = position }
    fn set_velocity(&mut self, velocity: DVec2) { self.velocity = velocity }
    fn set_acceleration(&mut self, acceleration: DVec2) { self.acceleration = acceleration }
}

/// A module of helper functions for computing on Body instances.
pub mod functions {
    use super::{Body2DTrait, HasPositionAndMassTrait, DVec2, Constants};

    /// Given two Bodies, A and B, calculate the gravitational force on Body A from Body B.
    pub fn calculate_gravity_between_bodies<B: HasPositionAndMassTrait>(a: &B, b: &B) -> DVec2 {
        // Calculate the displacement from A to B.
        let displacement = b.position() - a.position();
        let distance = displacement.length();

        // Add a catch for a zero distance (the Bodies are in the exact same position).
        // Just return the zero vector, so no force is returned.
        if distance.abs() < f64::EPSILON {
            return DVec2::ZERO;
        }

        // Calculate the scalar force from gravity.
        let force = Constants::G * ((a.mass() * b.mass()) / (distance * distance));

        // Normalize the displacement vector and multiply by force scalar.
        let direction = displacement.normalize();
        direction * force
    }

    /// Given Body A and a center of mass / total mass, calculate the gravitational force.
    pub fn calculate_gravity_from_point<B: HasPositionAndMassTrait>(a: &B, mass: f64, center_of_mass: DVec2) -> DVec2 {
        // Calculate the displacement from A to B.
        let displacement = center_of_mass - a.position();
        let distance = displacement.length();

        // Add a catch for a zero distance (the Bodies are in the exact same position).
        // Just return the zero vector, so no force is returned.
        if distance.abs() < f64::EPSILON {
            return DVec2::ZERO;
        }

        // Calculate the scalar force from gravity.
        let force = Constants::G * ((a.mass() * mass) / (distance * distance));

        // Normalize the displacement vector and multiply by force scalar.
        let direction = displacement.normalize();
        direction * force
    }

    /// Given a 2D force vector, apply that force to the Body.
    /// This updates its acceleration and velocity vectors.
    pub fn apply_force_to_body<B: Body2DTrait>(body: &mut B, force: DVec2, dt: f64) {
        // 1. Update position using current velocity and acceleration
        let position = body.position() + (body.velocity() * dt + 0.5 * body.acceleration() * dt * dt);
        body.set_position(position);

        // 2. Save old acceleration / compute new acceleration
        let old_acc = body.acceleration();
        let new_acc = force / body.mass();

        // 3. Average the old and new velocities and update velocity
        let velocity = body.velocity() + 0.5 * (old_acc + new_acc) * dt;
        body.set_velocity(velocity);

        // 4. Set the new acceleration
        body.set_acceleration(new_acc);
    }
}


/// A structure for defining the BodyBuilder.
/// BodyBuilder returns a Body object on the BodyBuilder::Build() call.
pub struct BodyBuilder {
    mass: f64,
    position: DVec2,
    velocity: DVec2,
    acceleration: DVec2
}

impl BodyBuilder {
    /// Set the mass of the Body from a f64.
    pub fn mass(&mut self, mass: f64) -> &mut Self {
        self.mass = mass;
        self
    }

    /// Set the position of the Body from a 2D vector.
    pub fn position_from_vector2D(&mut self, position: DVec2) -> &mut Self {
        self.position = position;
        self
    }

    /// Set the position of the Body from a 2D coordinate.
    pub fn position_from_point(&mut self, x: f64, y: f64) -> &mut Self {
        // Create a new DVec2 from the (x,y) point.
        self.position = DVec2::new(x, y);
        self
    }

    /// Set the velocity of the Body from a 2D vector.
    pub fn velocity_from_vector2D(&mut self, velocity: DVec2) -> &mut Self {
        self.velocity = velocity;
        self
    }

    /// Set the velocity of the Body from a 2D coordinate.
    pub fn velocity_from_point(&mut self, x_vel: f64, y_vel: f64) -> &mut Self {
        self.velocity = DVec2::new(x_vel, y_vel);
        self
    }

    /// Set the acceleration of the Body from a 2D vector.
    pub fn acceleration_from_vector2D(&mut self, acceleration: DVec2) -> &mut Self {
        self.acceleration = acceleration;
        self
    }

    /// Set the acceleration of the Body from a 2D vector.
    pub fn acceleration_from_point(&mut self, x_acc: f64, y_acc: f64) -> &mut Self {
        self.acceleration = DVec2::new(x_acc, y_acc);
        self
    }

    /// Return a new BodyBuilder instance with default values.
    /// All values default to 0.0 for f64's and DVec2::ZERO for vectors.
    pub fn new() -> Self {
        BodyBuilder {
            mass: 0.0,
            position: DVec2::ZERO,
            velocity: DVec2::ZERO,
            acceleration: DVec2::ZERO
        }
    }

    /// Build a Body instance using the values stored in the BodyBuilder up to this point.
    pub fn build(&mut self) -> Body {
        Body {
            mass: self.mass,
            position: self.position,
            velocity: self.velocity,
            acceleration: self.acceleration
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{BodyBuilder, Body2DTrait, HasPositionAndMassTrait, DVec2, functions};
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_body_builder() {
        let body = BodyBuilder::new()
            .mass(1e10)
            .position_from_point(5.0, -5.0)
            .velocity_from_point(10.0, 10.0)
            .acceleration_from_point(0.5, -1.0)
            .build();

        assert_eq!(body.mass(), 1e10);
        assert_eq!(body.position(), DVec2::new(5.0, -5.0));
        assert_eq!(body.velocity(), DVec2::new(10.0, 10.0));
        assert_eq!(body.acceleration(), DVec2::new(0.5, -1.0));
    }

    #[test]
    fn test_calculate_gravity_between_bodies() {
        let a = BodyBuilder::new()
            .mass(1e15)
            .position_from_point(0.0, 0.0)
            .build();

        let b = BodyBuilder::new()
            .mass(1e15)
            .position_from_point(1e10, 0.0)
            .build();

        // From gravity formula, the force of gravity between these two bodies should be
        // 0.6674 Newtons. It should be in the x-axis unit direction: <1,0>
        let force = functions::calculate_gravity_between_bodies(&a, &b);

        assert_abs_diff_eq!(force.x, 0.6674, epsilon = 1e-4);
        assert_eq!(force.y, 0.0);
    }

    #[test]
    fn test_apply_force_to_body() {
        let mut body = BodyBuilder::new()
            .mass(10.0)
            .build();

        let force = DVec2::new(10.0, 10.0);

        // From gravity formula, the force of gravity between these two bodies should be
        // 0.6674 Newtons. It should be in the x-axis unit direction: <1,0>
        functions::apply_force_to_body(&mut body, force, 1.0);

        assert_eq!(body.velocity(), DVec2::new(0.5, 0.5));
        assert_eq!(body.acceleration(), DVec2::new(1.0, 1.0));
    }
}