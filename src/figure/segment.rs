use super::ID;

/// The segment between the points `a` and `b`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Segment {
    pub a: ID,
    pub b: ID,
}

impl Segment {
    /// The segment between the points `a` and `b`.
    pub fn new(a: ID, b: ID) -> Self {
        Segment { a, b }
    }
}
