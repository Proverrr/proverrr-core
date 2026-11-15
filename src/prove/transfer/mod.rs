pub mod angle_sums;
pub mod base_transfer;
pub mod between_facts;
pub mod congruent_facts;
pub mod congruent_parts;
pub mod congruent_transfer;
pub mod isosceles;
pub mod midpoint_facts;
pub mod parallel_angle_transfer;
pub mod parallel_facts;
pub mod parallel_transfer;
pub mod parallelogram;
pub mod perpendicular_facts;
pub mod side_facts;
pub mod sum_transfer;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransferType {
    SumTransitive,
    TriangleAngleSum,
    BetweenFacts,
    SideFacts,
    IsoscelesAngles,
    IsoscelesSides,
    CongruentParts,
    CongruentSSS,
    CongruentSAS,
    CongruentASA,
    CongruentAAS,
    CongruentHL,
    ParallelTransitive,
    ParallelCorresponding,
    ParallelAlternateInterior,
    ParallelCoInterior,
    Parallelogram,
    PerpendicularFacts,
    MidpointFacts,
}
