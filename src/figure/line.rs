use super::ID;

/// A full line through the points `a` and `b`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Line {
    pub a: ID,
    pub b: ID,
}

impl Line {
    /// The line through the points `a` and `b`.
    pub fn new(a: ID, b: ID) -> Self {
        Line { a, b }
    }
}
