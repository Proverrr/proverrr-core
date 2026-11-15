use crate::figure::Figure;
use crate::prove::condition::{Condition, CongruentCondition, SumCondition};
use crate::prove::transfer::TransferType;
use crate::prove::transfer::base_transfer::BaseTransfer;
use crate::prove::transfer::congruent_facts::{ang_object, len_object};
use crate::prove::transfer::congruent_transfer::angle_at;
use crate::ratio::Ratio;

pub struct CongruentPartsTransfer;

// A congruence gives three equal sides and three equal angles.
impl BaseTransfer for CongruentPartsTransfer {
    fn transfer(&self, figure: &mut Figure) -> bool {
        let congruent: Vec<CongruentCondition> = figure
            .conditions()
            .iter()
            .filter_map(|c| match c {
                Condition::Congruent(c) => Some(*c),
                _ => None,
            })
            .collect();
        let mut candidates = Vec::new();
        for c in &congruent {
            let a = [c.a.a, c.a.b, c.a.c];
            let b = [c.b.a, c.b.b, c.b.c];
            for i in 0..3 {
                let j = (i + 1) % 3;
                let side = Condition::Sum(SumCondition::new(
                    vec![len_object(a[i], a[j])],
                    vec![len_object(b[i], b[j])],
                    Ratio::from(0),
                ));
                let angle = Condition::Sum(SumCondition::new(
                    vec![ang_object(
                        angle_at(a, i).0,
                        angle_at(a, i).1,
                        angle_at(a, i).2,
                    )],
                    vec![ang_object(
                        angle_at(b, i).0,
                        angle_at(b, i).1,
                        angle_at(b, i).2,
                    )],
                    Ratio::from(0),
                ));
                let fathers = vec![Condition::Congruent(*c)];
                candidates.push((side, fathers.clone(), TransferType::CongruentParts));
                candidates.push((angle, fathers, TransferType::CongruentParts));
            }
        }
        let mut changed = false;
        for (condition, fathers, describe) in candidates {
            changed |= figure.add_derived(condition, fathers, describe);
        }
        changed
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prove::solve::{Rule, solve};

    #[test]
    fn sss_gives_part_equalities() {
        let mut figure = Figure::new();
        let p: Vec<_> = (0..6).map(crate::figure::Point::new).collect();
        for (x, y, u, v) in [
            (p[0], p[1], p[3], p[4]),
            (p[1], p[2], p[4], p[5]),
            (p[2], p[0], p[5], p[3]),
        ] {
            figure.add_condition(Condition::Sum(SumCondition::new(
                vec![len_object(x, y)],
                vec![len_object(u, v)],
                Ratio::from(0),
            )));
        }
        let angle_target = Condition::Sum(SumCondition::new(
            vec![ang_object(p[2], p[0], p[1])],
            vec![ang_object(p[5], p[3], p[4])],
            Ratio::from(0),
        ));
        let proof = solve(&mut figure, &angle_target, &Rule::new()).expect("parts");
        assert!(proof.describe.contains(&TransferType::CongruentParts));
    }
}
