use crate::figure::Figure;
use crate::prove::condition::{Condition, ParallelCondition};
use crate::prove::transfer::TransferType;
use crate::prove::transfer::base_transfer::BaseTransfer;

pub struct ParallelTransfer;

// a // b and b // c give a // c.
impl BaseTransfer for ParallelTransfer {
    fn transfer(&self, figure: &mut Figure) -> bool {
        let parallels = collect_parallel(figure);
        let mut candidates = Vec::new();
        for i in 0..parallels.len() {
            for j in (i + 1)..parallels.len() {
                if let Some(condition) = combine(&parallels[i], &parallels[j]) {
                    let fathers = vec![
                        Condition::Parallel(parallels[i].clone()),
                        Condition::Parallel(parallels[j].clone()),
                    ];
                    candidates.push((Condition::Parallel(condition), fathers));
                }
            }
        }

        let mut changed = false;
        for (condition, fathers) in candidates {
            changed |= figure.add_derived(condition, fathers, TransferType::ParallelTransitive);
        }
        changed
    }
}

fn collect_parallel(figure: &Figure) -> Vec<ParallelCondition> {
    figure
        .conditions()
        .iter()
        .filter_map(|condition| match condition {
            Condition::Parallel(parallel) => Some(parallel.clone()),
            _ => None,
        })
        .collect()
}

fn combine(a: &ParallelCondition, b: &ParallelCondition) -> Option<ParallelCondition> {
    let pair = if a.a == b.a {
        (a.b, b.b)
    } else if a.a == b.b {
        (a.b, b.a)
    } else if a.b == b.a {
        (a.a, b.b)
    } else if a.b == b.b {
        (a.a, b.a)
    } else {
        return None;
    };
    if pair.0 == pair.1 {
        return None;
    }
    Some(ParallelCondition::new(pair.0, pair.1))
}
