use super::ID;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ObjectType {
    Length,
    Angle,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Object {
    pub kind: ObjectType,
    pub points: Vec<ID>,
}

impl Object {
    pub fn length(a: ID, b: ID) -> Self {
        Object {
            kind: ObjectType::Length,
            points: vec![a, b],
        }
    }

    pub fn angle(a: ID, b: ID, c: ID) -> Self {
        Object {
            kind: ObjectType::Angle,
            points: vec![a, b, c],
        }
    }
}
