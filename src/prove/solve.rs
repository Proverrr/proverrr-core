use crate::figure::Figure;
use crate::prove::condition::Condition;
use crate::prove::proof::Proof;
use crate::prove::transfer::base_transfer::BaseTransfer;
use crate::prove::transfer::parallel_transfer::ParallelTransfer;

pub struct Rule {
    pub parallel_transitive: bool,
}

impl Rule {
    pub fn new() -> Self {
        Rule {
            parallel_transitive: true,
        }
    }

    pub fn run(&self, figure: &mut Figure) -> bool {
        let mut changed = false;
        if self.parallel_transitive {
            changed |= ParallelTransfer.transfer(figure);
        }
        changed
    }
}

impl Default for Rule {
    fn default() -> Self {
        Rule::new()
    }
}

pub fn solve(figure: &mut Figure, target: &Condition, rule: &Rule) -> Option<Proof> {
    loop {
        if let Some(proof) = figure.proof_of(target) {
            return Some(proof.clone());
        }
        if !rule.run(figure) {
            return None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Rule, solve};
    use crate::figure::{Figure, PanLine, Segment};
    use crate::prove::condition::{Condition, ParallelCondition};
    use crate::prove::transfer::TransferType;

    #[test]
    fn solves_transitive_parallel() {
        let x = PanLine::Segment(Segment::new(0, 1));
        let y = PanLine::Segment(Segment::new(2, 3));
        let z = PanLine::Segment(Segment::new(4, 5));

        let mut figure = Figure::new();
        figure.add_condition(Condition::Parallel(ParallelCondition::new(x, y)));
        figure.add_condition(Condition::Parallel(ParallelCondition::new(y, z)));

        let target = Condition::Parallel(ParallelCondition::new(x, z));
        let proof = solve(&mut figure, &target, &Rule::new()).expect("target should be proven");
        assert_eq!(proof.describe, vec![TransferType::ParallelTransitive]);
    }
}
