use crate::figure::Point;

// Point mid is the midpoint of segment ac, stored with the endpoints in a
// fixed order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MidpointCondition {
    pub a: Point,
    pub mid: Point,
    pub b: Point,
}

impl MidpointCondition {
    pub fn new(a: Point, mid: Point, b: Point) -> Self {
        let (a, b) = if a <= b { (a, b) } else { (b, a) };
        MidpointCondition { a, mid, b }
    }
}
