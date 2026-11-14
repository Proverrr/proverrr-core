use crate::figure::Point;

// Three points lie on one line, stored in a fixed order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CollinearCondition {
    pub a: Point,
    pub b: Point,
    pub c: Point,
}

impl CollinearCondition {
    pub fn new(a: Point, b: Point, c: Point) -> Self {
        let mut pts = [a, b, c];
        pts.sort();
        CollinearCondition {
            a: pts[0],
            b: pts[1],
            c: pts[2],
        }
    }
}