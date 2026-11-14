use crate::figure::{PanLine, Point};

// Points a and b are on the same side of the line when same is true,
// on opposite sides when it is false. The pair is stored in a fixed order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SideCondition {
    pub line: PanLine,
    pub a: Point,
    pub b: Point,
    pub same: bool,
}

impl SideCondition {
    pub fn new(line: PanLine, a: Point, b: Point, same: bool) -> Self {
        let (a, b) = if a <= b { (a, b) } else { (b, a) };
        SideCondition { line, a, b, same }
    }
}