use crate::figure::PanLine;

// The two lines meet at a right angle, stored in a fixed order so equality
// is symmetric.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PerpendicularCondition {
    pub a: PanLine,
    pub b: PanLine,
}

impl PerpendicularCondition {
    pub fn new(a: PanLine, b: PanLine) -> Self {
        if a <= b {
            PerpendicularCondition { a, b }
        } else {
            PerpendicularCondition { a: b, b: a }
        }
    }
}
