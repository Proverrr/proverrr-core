use std::ops::Add;

use crate::prove::condition::Condition;
use crate::prove::transfer::TransferType;

// Every step proves conditions[i] from fathers[i] with describe[i].
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Proof {
    pub conditions: Vec<Condition>,
    pub fathers: Vec<Vec<Condition>>,
    pub describe: Vec<TransferType>,
}

impl Proof {
    pub fn new() -> Self {
        Proof::default()
    }

    pub fn push_step(
        &mut self,
        condition: Condition,
        fathers: Vec<Condition>,
        describe: TransferType,
    ) {
        if self.conditions.contains(&condition) {
            return;
        }
        self.conditions.push(condition);
        self.fathers.push(fathers);
        self.describe.push(describe);
    }
}

impl Add for Proof {
    type Output = Proof;

    fn add(self, other: Proof) -> Proof {
        let mut proof = self;
        for i in 0..other.conditions.len() {
            proof.push_step(
                other.conditions[i].clone(),
                other.fathers[i].clone(),
                other.describe[i],
            );
        }
        proof
    }
}
