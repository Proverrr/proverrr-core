use crate::figure::Point;

// Point mid lies strictly between a and b on their line, stored with the
// endpoints in a fixed order. Between-ness is the primitive that gives the
// symbolic rules their orientation facts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BetweenCondition {
    pub a: Point,
    pub mid: Point,
    pub b: Point,
}

impl BetweenCondition {
    pub fn new(a: Point, mid: Point, b: Point) -> Self {
        let (a, b) = if a <= b { (a, b) } else { (b, a) };
        BetweenCondition { a, mid, b }
    }
}
