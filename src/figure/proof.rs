use std::cell::RefCell;
use crate::ratio::Ratio;
use crate::transfer;
use super::*;
pub trait Condition{}
impl PartialEq for dyn Condition{
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}
#[derive(PartialEq)]
pub struct EqualCondition{
    a: object::Object,
    b: object::Object
}
impl Condition for EqualCondition{}
#[derive(PartialEq)]
pub struct ParallelCondition{
    a: Box<dyn PanLine>,
    b: Box<dyn PanLine>
}
impl Condition for ParallelCondition{}
#[derive(PartialEq)]
pub struct SumCondition{
    objects: Vec<object::Object>,
    sum_object: Vec<Option<object::Object,>>,
    sum_num: Ratio
}
impl Condition for SumCondition{}
#[derive(PartialEq)]
pub struct Proof{
    pub conditions: Vec<Box<dyn Condition>>,
    pub transfers: Vec<transfer::TransferType>,
}
