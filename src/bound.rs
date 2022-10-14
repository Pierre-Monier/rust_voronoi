#[derive(Debug)]
/// Represent the bound of the voronoi diagram.
pub struct DiagramBound {
    /// The width of the bound.
    pub width: f64,
    /// The height of the bound.
    pub height: f64,
}

impl DiagramBound {
    /// Creates a new DiagramBound.
    pub fn new(x: f64, y: f64) -> DiagramBound {
        DiagramBound {
            width: x,
            height: y,
        }
    }

    /// Returns a tuple (width, height) of the bound.
    pub fn to_tuple(&self) -> (f64, f64) {
        (self.width, self.height)
    }
}
