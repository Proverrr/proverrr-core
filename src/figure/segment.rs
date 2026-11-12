use super::ID;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Segment {
    pub a: ID,
    pub b: ID,
}

impl Segment {
    pub fn new(a: ID, b: ID) -> Self {
        Segment { a, b }
    }
}
