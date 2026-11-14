use crate::figure::Triangle;

// Triangle a is congruent to triangle b, with a.a<->b.a, a.b<->b.b, a.c<->b.c.
// The two triangles are stored in a fixed order so equality is symmetric.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CongruentCondition {
    pub a: Triangle,
    pub b: Triangle,
}

impl CongruentCondition {
    pub fn new(a: Triangle, b: Triangle) -> Self {
        if a <= b {
            CongruentCondition { a, b }
        } else {
            CongruentCondition { a: b, b: a }
        }
    }
}