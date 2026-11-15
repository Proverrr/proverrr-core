use crate::figure::Figure;
use crate::prove::condition::{Condition, SumCondition};
use crate::prove::transfer::TransferType;
use crate::prove::transfer::base_transfer::BaseTransfer;
use crate::ratio::Ratio;

// One reading of a Sum condition: sum(a) = sum(b) + c.
struct Sum {
    a: Vec<crate::figure::Object>,
    b: Vec<crate::figure::Object>,
    c: Ratio,
    source: Condition,
}

// A Sum condition is read in both directions, so the reverse of a known
// equality is also available to the chain rule.
fn reads(s: &SumCondition) -> Vec<Sum> {
    let forward = Sum {
        a: s.a.clone(),
        b: s.b_objects.clone(),
        c: s.b_const,
        source: Condition::Sum(s.clone()),
    };
    let backward = Sum {
        a: s.b_objects.clone(),
        b: s.a.clone(),
        c: -s.b_const,
        source: Condition::Sum(s.clone()),
    };
    vec![forward, backward]
}

pub struct SumTransfer;

impl BaseTransfer for SumTransfer {
    fn transfer(&self, figure: &mut Figure) -> bool {
        let reads = figure
            .conditions()
            .iter()
            .filter_map(|c| match c {
                Condition::Sum(s) => Some(reads(s)),
                _ => None,
            })
            .flatten()
            .collect::<Vec<Sum>>();

        let mut changed = false;
        for first in &reads {
            for second in &reads {
                if first.b != second.a {
                    continue;
                }
                if first.a == second.b && first.c == second.c {
                    continue;
                }
                let new = SumCondition::new(first.a.clone(), second.b.clone(), first.c + second.c);
                if new.a == new.b_objects && new.b_const.is_zero() {
                    continue;
                }
                let condition = Condition::Sum(new);
                changed |= figure.add_derived(
                    condition,
                    vec![first.source.clone(), second.source.clone()],
                    TransferType::SumTransitive,
                );
            }
        }
        changed
    }
}
