pub mod parallel_condition;
pub mod sum_condition;

pub use parallel_condition::ParallelCondition;
pub use sum_condition::SumCondition;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Condition {
    Parallel(ParallelCondition),
    Sum(SumCondition),
}
