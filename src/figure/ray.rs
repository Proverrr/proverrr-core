use super::ID;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ray {
    pub origin: ID,
    pub through: ID,
}

impl Ray {
    pub fn new(origin: ID, through: ID) -> Self {
        Ray { origin, through }
    }
}
