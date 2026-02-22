use glam::DVec2;

/// A trait representing objects that have a 2D position.
/// This is what we use for the BarnesHutTree, so any type wanting to be
/// inserted and managed by that tree must implement this trait.
pub trait Body2D: Clone {
    fn position(&self) -> DVec2;
    fn mass(&self) -> f64;
    fn distance<B: Body2D>(&self, other: &B) -> f64;
    fn apply_force_from<B: Body2D>(&mut self, other: &B);
}

/// Structure defining a Quadrant.
/// A quadrant represents a fourth of the space that a Node in the BarnesHutTree owns.
#[derive(Copy, Clone)]
pub struct Quadrant {
    start: DVec2,
    end: DVec2,
}

impl Quadrant {
    /// Return a new Quadrant instance.
    pub fn new(start: DVec2, end: DVec2) -> Self {
        Quadrant { start, end }
    }

    /// Return an array of four equally-sized subdivisions of this Quadrant.
    pub fn subdivide(&self) -> [Self; 4] {
        let subs = [
            Quadrant::new(
                self.start.clone(), 
                DVec2::new(self.end.x / 2.0, self.end.y / 2.0)
            ),
            Quadrant::new(
                DVec2::new(self.end.x / 2.0, self.start.y), 
                DVec2::new(self.end.x, self.end.y / 2.0)
            ),
            Quadrant::new(
                DVec2::new(self.start.x, self.end.y / 2.0), 
                DVec2::new(self.end.x / 2.0, self.end.y)
            ),
            Quadrant::new(
                DVec2::new(self.end.x / 2.0, self.end.y / 2.0), 
                self.end.clone()
            )
        ];
        subs
    }

    /// Determine if a given DVec2 is contained by this Quadrant.
    pub fn contains(&self, point: DVec2) -> bool {
        point.x >= self.start.x && point.x <= self.end.x && point.y >= self.start.y && point.y <= self.end.y
    }

    /// Returns the width of this Quadrant.
    pub fn width(&self) -> f64 {
        self.end.x - self.start.x
    }
}
