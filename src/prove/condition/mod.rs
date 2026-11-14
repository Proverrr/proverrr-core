pub mod collinear_condition;
pub mod congruent_condition;
pub mod parallel_condition;
pub mod side_condition;
pub mod sum_condition;

pub use collinear_condition::CollinearCondition;
pub use congruent_condition::CongruentCondition;
pub use parallel_condition::ParallelCondition;
pub use side_condition::SideCondition;
pub use sum_condition::SumCondition;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Condition {
    Parallel(ParallelCondition),
    Sum(SumCondition),
    Congruent(CongruentCondition),
    Side(SideCondition),
    Collinear(CollinearCondition),
}
