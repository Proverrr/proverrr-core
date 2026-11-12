pub mod base_transfer;
pub mod parallel_transfer;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransferType {
    ParallelTransitive,
}
