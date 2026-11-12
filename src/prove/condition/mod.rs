pub mod parallel_condition;
pub mod sum_condition;

pub trait Condition{}
impl PartialEq for dyn Condition{
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}