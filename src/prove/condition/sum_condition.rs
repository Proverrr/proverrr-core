use crate::figure::object;
use crate::prove::condition::Condition;
use crate::ratio::Ratio;

#[derive(PartialEq)]
pub struct SumCondition{
    //sum(a)=sum(b_objects)+sum(b_const)
    a: Vec<object::Object>,
    b_objects: Vec<object::Object>,
    b_const: Ratio
}
impl Condition for SumCondition {}