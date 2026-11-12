use super::ID;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Line {
    pub a: ID,
    pub b: ID,
}

impl Line {
    pub fn new(a: ID, b: ID) -> Self {
        Line { a, b }
    }
}
