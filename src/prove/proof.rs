use std::collections::HashSet;
use std::ops::Add;
pub(crate) use crate::prove::condition::Condition;
use crate::prove::transfer;
use crate::prove::transfer::TransferType;

#[derive(PartialEq)]
pub struct Proof{
    pub conditions: Vec<Box<dyn Condition>>,//Dependencies of this proof
    pub fathers: Vec<Vec<Box<dyn Condition>>>,//Dependencies of each, to recover the proving process
    pub describe: Vec<TransferType>//Transfer type of each, to recover the proving process
}
impl Add for Proof {
    type Output = Proof;
    fn add(self, other: &Proof) -> Proof {
        let mut hashset: HashSet<dyn Condition>;
        let mut conditions = self.conditions;
        let mut fathers = self.fathers;
        let mut discribe = self.describe;
        for i in 0..other.conditions.len(){
            if !hashset.contains(&other.conditions[i]) {
                conditions.push(other.conditions[i]);
                fathers.push(other.fathers[i]);
                discribe.push(other.describe[i]);
                hashset.insert(other.conditions[i]);
            }
        }
        Proof{conditions, fathers, describe: discribe }
    }
}