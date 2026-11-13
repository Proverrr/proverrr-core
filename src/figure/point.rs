use crate::ratio::Ratio;

/// A point with exact rational coordinates.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Point {
    pub x: Ratio,
    pub y: Ratio,
}

impl Point {
    /// The point at the coordinates `x` and `y`.
    pub fn new(x: Ratio, y: Ratio) -> Self {
        Point { x, y }
    }

    /// The point `(0, 0)`.
    pub fn origin() -> Self {
        Point::new(Ratio::from(0), Ratio::from(0))
    }
}
