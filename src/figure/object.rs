use super::Point;

/// What kind of quantity an [`Object`] measures.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ObjectType {
    /// The distance between two points.
    Length,
    /// The angle at the middle point of three.
    Angle,
}

/// A measurable quantity of a figure, such as a length or an angle.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Object {
    pub kind: ObjectType,
    pub points: Vec<Point>,
}

impl Object {
    /// The length of the segment between the points `a` and `b`.
    pub fn length(a: Point, b: Point) -> Self {
        Object {
            kind: ObjectType::Length,
            points: vec![a, b],
        }
    }

    /// The angle `abc`, with `b` as the vertex.
    pub fn angle(a: Point, b: Point, c: Point) -> Self {
        Object {
            kind: ObjectType::Angle,
            points: vec![a, b, c],
        }
    }
}