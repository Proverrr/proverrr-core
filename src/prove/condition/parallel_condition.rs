use crate::figure::PanLine;

// A is parallel to B, stored in a fixed order so equality is symmetric.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParallelCondition {
    pub a: PanLine,
    pub b: PanLine,
}

impl ParallelCondition {
    pub fn new(a: PanLine, b: PanLine) -> Self {
        if a <= b {
            ParallelCondition { a, b }
        } else {
            ParallelCondition { a: b, b: a }
        }
    }
}
