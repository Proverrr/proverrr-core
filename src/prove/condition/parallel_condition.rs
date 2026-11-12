use crate::figure::PanLine;
use super::Condition;

#[derive(PartialEq)]
pub struct ParallelCondition{
    // A(Pan-Line) is parallel to B(Pan-Line)
    a: Box<dyn PanLine>,
    b: Box<dyn PanLine>
}
impl Condition for ParallelCondition{}