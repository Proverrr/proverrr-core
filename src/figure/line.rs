use super::Point;

/// A full line through the points `a` and `b`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Line {
    pub a: Point,
    pub b: Point,
}

impl Line {
    /// The line through the points `a` and `b`.
    pub fn new(a: Point, b: Point) -> Self {
        Line { a, b }
    }
}