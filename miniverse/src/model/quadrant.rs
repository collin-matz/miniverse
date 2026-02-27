use glam::DVec2;

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
        let mid = DVec2::new(
            (self.start.x + self.end.x) / 2.0,
            (self.start.y + self.end.y) / 2.0
        );

        [
            Quadrant::new(self.start, DVec2::new(mid.x, mid.y)),
            Quadrant::new(DVec2::new(mid.x, self.start.y), DVec2::new(self.end.x, mid.y)),
            Quadrant::new(DVec2::new(self.start.x, mid.y), DVec2::new(mid.x, self.end.y)),
            Quadrant::new(mid, self.end)
        ]
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
