use crate::figure::{Figure, Point, Triangle};
use crate::prove::condition::{Condition, CongruentCondition};
use crate::prove::transfer::TransferType;
use crate::prove::transfer::base_transfer::BaseTransfer;
use crate::prove::transfer::congruent_facts::Facts;

#[derive(Clone, Copy)]
pub enum Criterion {
    Sss,
    Sas,
    Asa,
    Aas,
    Hl,
}

impl Criterion {
    pub fn transfer_type(self) -> TransferType {
        match self {
            Criterion::Sss => TransferType::CongruentSSS,
            Criterion::Sas => TransferType::CongruentSAS,
            Criterion::Asa => TransferType::CongruentASA,
            Criterion::Aas => TransferType::CongruentAAS,
            Criterion::Hl => TransferType::CongruentHL,
        }
    }
}

// A required part of a congruence, indexed by vertex in the two triangles.
#[derive(Clone, Copy)]
pub enum Part {
    Side(usize, usize),
    Angle(usize),
    Right(usize),
}

pub const PERMS: [[usize; 3]; 6] = [
    [0, 1, 2],
    [0, 2, 1],
    [1, 0, 2],
    [1, 2, 0],
    [2, 0, 1],
    [2, 1, 0],
];

pub fn angle_at(t: [Point; 3], i: usize) -> (Point, Point, Point) {
    (t[(i + 2) % 3], t[i], t[(i + 1) % 3])
}

pub struct CongruentTransfer {
    pub criteria: Vec<Criterion>,
}

impl BaseTransfer for CongruentTransfer {
    fn transfer(&self, figure: &mut Figure) -> bool {
        if self.criteria.is_empty() {
            return false;
        }
        let facts = Facts::from_figure(figure);
        let tris = facts.triangles();
        let mut candidates = Vec::new();
        for i in 0..tris.len() {
            for j in (i + 1)..tris.len() {
                if !facts.any_side_equal(tris[i], tris[j]) {
                    continue;
                }
                for perm in PERMS {
                    let b = [tris[j][perm[0]], tris[j][perm[1]], tris[j][perm[2]]];
                    for &c in &self.criteria {
                        if let Some(fathers) = check(c, &facts, tris[i], b) {
                            let ta = Triangle::new(tris[i][0], tris[i][1], tris[i][2]);
                            let tb = Triangle::new(b[0], b[1], b[2]);
                            let cond = Condition::Congruent(CongruentCondition::new(ta, tb));
                            candidates.push((cond, fathers, c.transfer_type()));
                        }
                    }
                }
            }
        }
        let mut changed = false;
        for (cond, fathers, describe) in candidates {
            changed |= figure.add_derived(cond, fathers, describe);
        }
        changed
    }
}

// Each case lists the parts that must match under the correspondence a[i] <-> b[i].
pub fn check(c: Criterion, facts: &Facts, a: [Point; 3], b: [Point; 3]) -> Option<Vec<Condition>> {
    let cases: &[&[Part]] = match c {
        Criterion::Sss => &[&[Part::Side(0, 1), Part::Side(1, 2), Part::Side(2, 0)]],
        Criterion::Sas => &[
            &[Part::Side(0, 1), Part::Side(1, 2), Part::Angle(1)],
            &[Part::Side(1, 2), Part::Side(2, 0), Part::Angle(2)],
            &[Part::Side(2, 0), Part::Side(0, 1), Part::Angle(0)],
        ],
        Criterion::Asa => &[
            &[Part::Angle(0), Part::Angle(1), Part::Side(0, 1)],
            &[Part::Angle(1), Part::Angle(2), Part::Side(1, 2)],
            &[Part::Angle(2), Part::Angle(0), Part::Side(2, 0)],
        ],
        Criterion::Aas => &[
            &[Part::Angle(0), Part::Angle(1), Part::Side(1, 2)],
            &[Part::Angle(0), Part::Angle(1), Part::Side(2, 0)],
            &[Part::Angle(1), Part::Angle(2), Part::Side(2, 0)],
            &[Part::Angle(1), Part::Angle(2), Part::Side(0, 1)],
            &[Part::Angle(2), Part::Angle(0), Part::Side(0, 1)],
            &[Part::Angle(2), Part::Angle(0), Part::Side(1, 2)],
        ],
        Criterion::Hl => &[
            &[Part::Right(0), Part::Side(1, 2), Part::Side(0, 1)],
            &[Part::Right(0), Part::Side(1, 2), Part::Side(2, 0)],
            &[Part::Right(1), Part::Side(2, 0), Part::Side(0, 1)],
            &[Part::Right(1), Part::Side(2, 0), Part::Side(1, 2)],
            &[Part::Right(2), Part::Side(0, 1), Part::Side(1, 2)],
            &[Part::Right(2), Part::Side(0, 1), Part::Side(2, 0)],
        ],
    };
    for case in cases {
        let mut fathers = Vec::new();
        let ok = case.iter().all(|part| match part {
            Part::Side(i, j) => facts.side_equal(a[*i], a[*j], b[*i], b[*j], &mut fathers),
            Part::Angle(i) => facts.angle_equal(angle_at(a, *i), angle_at(b, *i), &mut fathers),
            Part::Right(i) => facts.right_equal(angle_at(a, *i), angle_at(b, *i), &mut fathers),
        });
        if ok {
            return Some(fathers);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::figure::Object;
    use crate::prove::condition::SumCondition;
    use crate::prove::solve::{Rule, solve};
    use crate::ratio::Ratio;

    fn eq(x: Object, y: Object) -> Condition {
        Condition::Sum(SumCondition::new(vec![x], vec![y], Ratio::from(0)))
    }

    fn add_points(figure: &mut Figure, n: usize) -> Vec<Point> {
        (0..n).map(|_| figure.add_point()).collect()
    }

    #[test]
    fn solves_sss() {
        let mut figure = Figure::new();
        let p = add_points(&mut figure, 6);
        figure.add_condition(eq(Object::length(p[0], p[1]), Object::length(p[3], p[4])));
        figure.add_condition(eq(Object::length(p[1], p[2]), Object::length(p[4], p[5])));
        figure.add_condition(eq(Object::length(p[2], p[0]), Object::length(p[5], p[3])));
        let target = Condition::Congruent(CongruentCondition::new(
            Triangle::new(p[0], p[1], p[2]),
            Triangle::new(p[3], p[4], p[5]),
        ));
        let proof = solve(&mut figure, &target, &Rule::new()).expect("sss should be proven");
        assert_eq!(proof.describe, vec![TransferType::CongruentSSS]);
    }

    #[test]
    fn solves_hl() {
        let mut figure = Figure::new();
        let p = add_points(&mut figure, 6);
        figure.add_condition(Condition::Sum(SumCondition::new(
            vec![Object::angle(p[0], p[1], p[2])],
            vec![],
            Ratio::from(90),
        )));
        figure.add_condition(Condition::Sum(SumCondition::new(
            vec![Object::angle(p[3], p[4], p[5])],
            vec![],
            Ratio::from(90),
        )));
        figure.add_condition(eq(Object::length(p[0], p[2]), Object::length(p[3], p[5])));
        figure.add_condition(eq(Object::length(p[0], p[1]), Object::length(p[3], p[4])));
        let target = Condition::Congruent(CongruentCondition::new(
            Triangle::new(p[0], p[1], p[2]),
            Triangle::new(p[3], p[4], p[5]),
        ));
        let proof = solve(&mut figure, &target, &Rule::new()).expect("hl should be proven");
        assert_eq!(proof.describe, vec![TransferType::CongruentHL]);
    }
}