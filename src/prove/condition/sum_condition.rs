use crate::figure::Object;
use crate::ratio::Ratio;

// sum(a) = sum(b_objects) + b_const
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SumCondition {
    pub a: Vec<Object>,
    pub b_objects: Vec<Object>,
    pub b_const: Ratio,
}

impl SumCondition {
    pub fn new(a: Vec<Object>, b_objects: Vec<Object>, b_const: Ratio) -> Self {
        SumCondition {
            a,
            b_objects,
            b_const,
        }
    }
}
