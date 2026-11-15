use crate::figure::Figure;
use crate::prove::condition::{Condition, SumCondition};
use crate::prove::transfer::TransferType;
use crate::prove::transfer::base_transfer::BaseTransfer;
use crate::prove::transfer::congruent_facts::ang_object;
use crate::ratio::Ratio;

pub struct AngleSumTransfer;

// The three angles of any triangle sum to 180 degrees; the fact is
// unconditional, so it is derived for every triple of distinct points.
impl BaseTransfer for AngleSumTransfer {
    fn transfer(&self, figure: &mut Figure) -> bool {
        let points: Vec<_> = figure.points().to_vec();
        let mut candidates = Vec::new();
        for i in 0..points.len() {
            for j in (i + 1)..points.len() {
                for k in (j + 1)..points.len() {
                    let condition = Condition::Sum(SumCondition::new(
                        vec![
                            ang_object(points[i], points[j], points[k]),
                            ang_object(points[j], points[k], points[i]),
                            ang_object(points[k], points[i], points[j]),
                        ],
                        vec![],
                        Ratio::from(180),
                    ));
                    candidates.push(condition);
                }
            }
        }

        let mut changed = false;
        for condition in candidates {
            changed |= figure.add_derived(condition, Vec::new(), TransferType::TriangleAngleSum);
        }
        changed
    }
}
