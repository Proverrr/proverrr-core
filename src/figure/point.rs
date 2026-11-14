use super::ID;

/// An abstract point, identified by its [`ID`] inside a figure.
///
/// Points carry no coordinates; the figure hands them out with
/// [`crate::figure::Figure::add_point`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point {
    id: ID,
}

impl Point {
    pub fn new(id: ID) -> Self {
        Point { id }
    }

    /// The index of this point inside its figure.
    pub fn id(&self) -> ID {
        self.id
    }
}