pub mod collinear_condition;
pub mod congruent_condition;
pub mod between_condition;
pub mod midpoint_condition;
pub mod parallel_condition;
pub mod perpendicular_condition;
pub mod side_condition;
pub mod sum_condition;

pub use collinear_condition::CollinearCondition;
pub use between_condition::BetweenCondition;
pub use congruent_condition::CongruentCondition;
pub use midpoint_condition::MidpointCondition;
pub use parallel_condition::ParallelCondition;
pub use perpendicular_condition::PerpendicularCondition;
pub use side_condition::SideCondition;
pub use sum_condition::SumCondition;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Condition {
    Parallel(ParallelCondition),
    Perpendicular(PerpendicularCondition),
    Sum(SumCondition),
    Congruent(CongruentCondition),
    Side(SideCondition),
    Collinear(CollinearCondition),
    Between(BetweenCondition),
    Midpoint(MidpointCondition),
}
