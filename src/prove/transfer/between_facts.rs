use crate::figure::Figure;
use crate::prove::condition::{
    BetweenCondition, CollinearCondition, Condition, SumCondition,
};
use crate::prove::transfer::TransferType;
use crate::prove::transfer::base_transfer::BaseTransfer;
use crate::prove::transfer::congruent_facts::ang_object;
use crate::ratio::Ratio;

pub struct BetweenTransfer;

// What follows from between-ness: collinearity, linear pairs, vertical
// angles and exterior angles. All of these are sound for degenerate
// positions too, because the rays are known opposite.
impl BaseTransfer for BetweenTransfer {
    fn transfer(&self, figure: &mut Figure) -> bool {
        let betweens: Vec<BetweenCondition> = figure
            .conditions()
            .iter()
            .filter_map(|c| match c {
                Condition::Between(b) => Some(*b),
                _ => None,
            })
            .collect();
        let points: Vec<_> = figure.points().to_vec();

        let mut candidates = Vec::new();
        for b in &betweens {
            let collinear = Condition::Collinear(CollinearCondition::new(b.a, b.mid, b.b));
            candidates.push((collinear, vec![Condition::Between(*b)]));

            // Linear pairs: the rays from mid to a and to b are opposite, so
            // they surround a straight angle with every other ray.
            for p in &points {
                if *p == b.mid {
                    continue;
                }
                let pair = Condition::Sum(SumCondition::new(
                    vec![ang_object(b.a, b.mid, *p), ang_object(*p, b.mid, b.b)],
                    vec![],
                    Ratio::from(180),
                ));
                candidates.push((pair, vec![Condition::Between(*b)]));
            }

            // Vertical angles against another pair of opposite rays.
            for other in &betweens {
                if other.mid != b.mid {
                    continue;
                }
                if other.a == b.a || other.a == b.b || other.b == b.a || other.b == b.b {
                    continue;
                }
                for (x, y) in [(b.a, b.b), (b.b, b.a)] {
                    for (u, v) in [(other.a, other.b), (other.b, other.a)] {
                        let cond = Condition::Sum(SumCondition::new(
                            vec![ang_object(x, b.mid, u)],
                            vec![ang_object(y, b.mid, v)],
                            Ratio::from(0),
                        ));
                        candidates.push((
                            cond,
                            vec![Condition::Between(*b), Condition::Between(*other)],
                        ));
                    }
                }
            }

            // Exterior angle: triangle (a, mid, p) with the side a-mid
            // extended through mid, so mid lies between a and b.
            for p in &points {
                if *p == b.mid || *p == b.a || *p == b.b {
                    continue;
                }
                let ext = Condition::Sum(SumCondition::new(
                    vec![ang_object(b.b, b.mid, *p)],
                    vec![ang_object(b.mid, b.a, *p), ang_object(b.a, *p, b.mid)],
                    Ratio::from(0),
                ));
                candidates.push((ext, vec![Condition::Between(*b)]));
            }
        }

        let mut changed = false;
        for (condition, fathers) in candidates {
            changed |= figure.add_derived(condition, fathers, TransferType::BetweenFacts);
        }
        changed
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::figure::Object;
    use crate::prove::solve::{Rule, solve};

    // a--m--b on one line, c off it: the angles at m form a linear pair and
    // vertical angles.
    #[test]
    fn linear_pair_and_vertical_angles() {
        let mut figure = Figure::new();
        let a = figure.add_point();
        let m = figure.add_point();
        let b = figure.add_point();
        let c = figure.add_point();
        let d = figure.add_point();
        figure.add_condition(Condition::Between(BetweenCondition::new(a, m, b)));
        figure.add_condition(Condition::Between(BetweenCondition::new(c, m, d)));

        let pair = Condition::Sum(SumCondition::new(
            vec![ang_object(a, m, c), ang_object(c, m, b)],
            vec![],
            Ratio::from(180),
        ));
        let proof = solve(&mut figure, &pair, &Rule::new()).expect("linear pair");
        assert!(proof.describe.contains(&TransferType::BetweenFacts));

        let vertical = Condition::Sum(SumCondition::new(
            vec![Object::angle(a, m, c)],
            vec![Object::angle(b, m, d)],
            Ratio::from(0),
        ));
        let proof = solve(&mut figure, &vertical, &Rule::new()).expect("vertical angles");
        assert!(proof.describe.contains(&TransferType::BetweenFacts));
    }

    // Exterior angle of triangle (a, m, c) at m equals the two remote angles.
    #[test]
    fn exterior_angle() {
        let mut figure = Figure::new();
        let a = figure.add_point();
        let m = figure.add_point();
        let b = figure.add_point();
        let c = figure.add_point();
        figure.add_condition(Condition::Between(BetweenCondition::new(a, m, b)));

        let target = Condition::Sum(SumCondition::new(
            vec![ang_object(b, m, c)],
            vec![ang_object(m, a, c), ang_object(a, c, m)],
            Ratio::from(0),
        ));
        let proof = solve(&mut figure, &target, &Rule::new()).expect("exterior angle");
        assert!(proof.describe.contains(&TransferType::BetweenFacts));
    }
}
