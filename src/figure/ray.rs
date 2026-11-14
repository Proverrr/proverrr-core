use super::Point;

/// The ray that starts at `origin` and goes through `through`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ray {
    pub origin: Point,
    pub through: Point,
}

impl Ray {
    /// The ray that starts at `origin` and goes through `through`.
    pub fn new(origin: Point, through: Point) -> Self {
        Ray { origin, through }
    }
}