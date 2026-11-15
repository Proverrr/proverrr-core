use crate::figure::{Figure, ObjectType};
use crate::prove::condition::{BetweenCondition, Condition, MidpointCondition, SumCondition};
use crate::prove::transfer::TransferType;
use crate::prove::transfer::base_transfer::BaseTransfer;
use crate::prove::transfer::congruent_facts::len_object;
use crate::ratio::Ratio;

pub struct MidpointTransfer;

// A midpoint lies between the endpoints and splits the segment into two
// equal parts; conversely, equal parts with between-ness make a midpoint.
// Both directions are sound without extra assumptions.
impl BaseTransfer for MidpointTransfer {
    fn transfer(&self, figure: &mut Figure) -> bool {
        let midpoints: Vec<MidpointCondition> = figure
            .conditions()
            .iter()
            .filter_map(|c| match c {
                Condition::Midpoint(m) => Some(*m),
                _ => None,
            })
            .collect();
        let sums: Vec<SumCondition> = figure
            .conditions()
            .iter()
            .filter_map(|c| match c {
                Condition::Sum(s) => Some(s.clone()),
                _ => None,
            })
            .collect();
        let betweens: Vec<BetweenCondition> = figure
            .conditions()
            .iter()
            .filter_map(|c| match c {
                Condition::Between(b) => Some(*b),
                _ => None,
            })
            .collect();

        let mut candidates = Vec::new();
        for m in &midpoints {
            let split = Condition::Sum(SumCondition::new(
                vec![len_object(m.a, m.mid)],
                vec![len_object(m.mid, m.b)],
                Ratio::from(0),
            ));
            let between = Condition::Between(BetweenCondition::new(m.a, m.mid, m.b));
            let father = Condition::Midpoint(*m);
            candidates.push((split, vec![father.clone()]));
            candidates.push((between, vec![father]));
        }
        for s in &sums {
            if let Some((a, m, b)) = halves(&s) {
                for b_ in &betweens {
                    let pts = [b_.a, b_.mid, b_.b];
                    if pts.contains(&a) && pts.contains(&m) && pts.contains(&b) {
                        let condition = Condition::Midpoint(MidpointCondition::new(a, m, b));
                        let fathers = vec![
                            Condition::Sum(s.clone()),
                            Condition::Between(*b_),
                        ];
                        candidates.push((condition, fathers));
                    }
                }
            }
        }

        let mut changed = false;
        for (condition, fathers) in candidates {
            changed |= figure.add_derived(condition, fathers, TransferType::MidpointFacts);
        }
        changed
    }
}

// length(a, m) = length(m, b): m halves the segment ab.
fn halves(s: &SumCondition) -> Option<(crate::figure::Point, crate::figure::Point, crate::figure::Point)> {
    if s.a.len() != 1 || s.b_objects.len() != 1 || !s.b_const.is_zero() {
        return None;
    }
    let (x, y) = (&s.a[0], &s.b_objects[0]);
    if x.kind != ObjectType::Length || y.kind != ObjectType::Length {
        return None;
    }
    let (p1, p2) = (x.points[0], x.points[1]);
    let (q1, q2) = (y.points[0], y.points[1]);
    for (m, a, b) in [(p1, p2, q2), (p1, p2, q1), (p2, p1, q2), (p2, p1, q1)] {
        let other = if m == q1 { q2 } else if m == q2 { q1 } else { continue };
        if a == b || a == m || b == m || other != b {
            continue;
        }
        return Some((a, m, b));
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::figure::Point;
    use crate::prove::solve::{Rule, solve};

    #[test]
    fn midpoint_from_equal_halves() {
        let mut figure = Figure::new();
        let a = Point::new(0);
        let m = Point::new(1);
        let b = Point::new(2);
        figure.add_condition(Condition::Between(BetweenCondition::new(a, m, b)));
        figure.add_condition(Condition::Sum(SumCondition::new(
            vec![len_object(a, m)],
            vec![len_object(m, b)],
            Ratio::from(0),
        )));

        let target = Condition::Midpoint(MidpointCondition::new(a, m, b));
        let proof = solve(&mut figure, &target, &Rule::new()).expect("midpoint");
        assert!(proof.describe.contains(&TransferType::MidpointFacts));
    }

    // And back: a midpoint gives the equal halves and the between-ness.
    #[test]
    fn midpoint_gives_halves() {
        let mut figure = Figure::new();
        let a = Point::new(0);
        let m = Point::new(1);
        let b = Point::new(2);
        figure.add_condition(Condition::Midpoint(MidpointCondition::new(a, m, b)));

        let target = Condition::Sum(SumCondition::new(
            vec![len_object(a, m)],
            vec![len_object(m, b)],
            Ratio::from(0),
        ));
        let proof = solve(&mut figure, &target, &Rule::new()).expect("halves");
        assert!(proof.describe.contains(&TransferType::MidpointFacts));

        let target = Condition::Between(BetweenCondition::new(a, m, b));
        let proof = solve(&mut figure, &target, &Rule::new()).expect("between");
        assert!(proof.describe.contains(&TransferType::MidpointFacts));
    }
}
