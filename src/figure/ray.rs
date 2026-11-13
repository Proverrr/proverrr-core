use super::ID;

/// The ray that starts at `origin` and goes through `through`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ray {
    pub origin: ID,
    pub through: ID,
}

impl Ray {
    /// The ray that starts at `origin` and goes through `through`.
    pub fn new(origin: ID, through: ID) -> Self {
        Ray { origin, through }
    }
}
