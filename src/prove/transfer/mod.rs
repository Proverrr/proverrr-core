pub mod base_transfer;
pub mod congruent_facts;
pub mod congruent_transfer;
pub mod parallel_angle_transfer;
pub mod parallel_facts;
pub mod parallel_transfer;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransferType {
    ParallelTransitive,
    ParallelCorresponding,
    ParallelAlternateInterior,
    ParallelCoInterior,
    CongruentSSS,
    CongruentSAS,
    CongruentASA,
    CongruentAAS,
    CongruentHL,
}