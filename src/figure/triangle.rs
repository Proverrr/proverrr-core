use super::Point;

/// A triangle defined by its three vertices.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Triangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
}

impl Triangle {
    /// The triangle with vertices `a`, `b` and `c`.
    pub fn new(a: Point, b: Point, c: Point) -> Self {
        Triangle { a, b, c }
    }
}