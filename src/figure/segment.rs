use super::Point;

/// The segment between the points `a` and `b`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Segment {
    pub a: Point,
    pub b: Point,
}

impl Segment {
    /// The segment between the points `a` and `b`.
    pub fn new(a: Point, b: Point) -> Self {
        Segment { a, b }
    }
}