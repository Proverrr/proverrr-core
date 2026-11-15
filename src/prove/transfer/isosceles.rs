use crate::figure::{Figure, Object, ObjectType};
use crate::prove::condition::{Condition, SumCondition};
use crate::prove::transfer::TransferType;
use crate::prove::transfer::base_transfer::BaseTransfer;
use crate::prove::transfer::congruent_facts::{ang_object, len_object};
use crate::ratio::Ratio;

pub struct IsoscelesTransfer;

// Equal sides from one apex give equal base angles, and equal base angles
// give equal sides. Both directions assume nondegenerate triangles.
impl BaseTransfer for IsoscelesTransfer {
    fn transfer(&self, figure: &mut Figure) -> bool {
        let sums: Vec<SumCondition> = figure
            .conditions()
            .iter()
            .filter_map(|c| match c {
                Condition::Sum(s) => Some(s.clone()),
                _ => None,
            })
            .collect();

        let mut candidates = Vec::new();
        for s in &sums {
            if let Some((apex, u1, u2)) = equal_sides(s) {
                let condition = Condition::Sum(SumCondition::new(
                    vec![ang_object(apex, u1, u2)],
                    vec![ang_object(apex, u2, u1)],
                    Ratio::from(0),
                ));
                candidates.push((
                    condition,
                    vec![Condition::Sum(s.clone())],
                    TransferType::IsoscelesAngles,
                ));
            }
            if let Some((apex, u1, u2)) = equal_base_angles(s) {
                let condition = Condition::Sum(SumCondition::new(
                    vec![len_object(apex, u1)],
                    vec![len_object(apex, u2)],
                    Ratio::from(0),
                ));
                candidates.push((
                    condition,
                    vec![Condition::Sum(s.clone())],
                    TransferType::IsoscelesSides,
                ));
            }
        }

        let mut changed = false;
        for (condition, fathers, describe) in candidates {
            changed |= figure.add_derived(condition, fathers, describe);
        }
        changed
    }
}

// length(p,u1) = length(p,u2): u1 and u2 are the base, p the apex.
fn equal_sides(
    s: &SumCondition,
) -> Option<(
    crate::figure::Point,
    crate::figure::Point,
    crate::figure::Point,
)> {
    let (x, y) = single_pair(s)?;
    if x.kind != ObjectType::Length || y.kind != ObjectType::Length {
        return None;
    }
    let (p, q) = (x.points[0], x.points[1]);
    let (r, t) = (y.points[0], y.points[1]);
    for (a, b, c, d) in [(p, q, r, t), (p, q, t, r)] {
        if a == c && b != d {
            return Some((a, b, d));
        }
        if b == d && a != c {
            return Some((b, a, c));
        }
    }
    None
}

// angle(p,u1,u2) = angle(p,u2,u1): equal angles at u1 and u2, so the sides
// from p to u1 and p to u2 are equal.
fn equal_base_angles(
    s: &SumCondition,
) -> Option<(
    crate::figure::Point,
    crate::figure::Point,
    crate::figure::Point,
)> {
    let (x, y) = single_pair(s)?;
    if x.kind != ObjectType::Angle || y.kind != ObjectType::Angle {
        return None;
    }
    for (a, b, c, d, e, f) in [
        (
            x.points[0],
            x.points[1],
            x.points[2],
            y.points[0],
            y.points[1],
            y.points[2],
        ),
        (
            y.points[0],
            y.points[1],
            y.points[2],
            x.points[0],
            x.points[1],
            x.points[2],
        ),
    ] {
        if a == d && b == f && c == e && b != c {
            return Some((a, b, c));
        }
    }
    None
}

fn single_pair(s: &SumCondition) -> Option<(&Object, &Object)> {
    if s.a.len() == 1 && s.b_objects.len() == 1 && s.b_const.is_zero() {
        Some((&s.a[0], &s.b_objects[0]))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::figure::Point;
    use crate::figure::{Figure, Object};
    use crate::prove::solve::{Rule, solve};

    #[test]
    fn derives_base_angles() {
        let mut figure = Figure::new();
        let a = Point::new(0);
        let b = Point::new(1);
        let c = Point::new(2);
        figure.add_condition(Condition::Sum(SumCondition::new(
            vec![Object::length(a, b)],
            vec![Object::length(a, c)],
            Ratio::from(0),
        )));
        let target = Condition::Sum(SumCondition::new(
            vec![Object::angle(a, b, c)],
            vec![Object::angle(a, c, b)],
            Ratio::from(0),
        ));
        let proof = solve(&mut figure, &target, &Rule::new()).expect("base angles");
        assert!(proof.describe.contains(&TransferType::IsoscelesAngles));
    }

    #[test]
    fn derives_sides_from_base_angles() {
        let mut figure = Figure::new();
        let a = Point::new(0);
        let b = Point::new(1);
        let c = Point::new(2);
        figure.add_condition(Condition::Sum(SumCondition::new(
            vec![Object::angle(a, b, c)],
            vec![Object::angle(a, c, b)],
            Ratio::from(0),
        )));
        let target = Condition::Sum(SumCondition::new(
            vec![Object::length(a, b)],
            vec![Object::length(a, c)],
            Ratio::from(0),
        ));
        let proof = solve(&mut figure, &target, &Rule::new()).expect("equal sides");
        assert!(proof.describe.contains(&TransferType::IsoscelesSides));
    }
}
