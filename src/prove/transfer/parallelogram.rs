use crate::figure::{Figure, PanLine, Point};
use crate::prove::condition::{CollinearCondition, Condition, ParallelCondition, SumCondition};
use crate::prove::transfer::TransferType;
use crate::prove::transfer::base_transfer::BaseTransfer;
use crate::prove::transfer::congruent_facts::{ang_object, len_object};
use crate::ratio::Ratio;

pub struct ParallelogramTransfer;

// A quadrilateral with both pairs of opposite sides parallel has equal
// opposite sides and equal opposite angles.
impl BaseTransfer for ParallelogramTransfer {
    fn transfer(&self, figure: &mut Figure) -> bool {
        let parallels: Vec<ParallelCondition> = figure
            .conditions()
            .iter()
            .filter_map(|c| match c {
                Condition::Parallel(p) => Some(p.clone()),
                _ => None,
            })
            .collect();

        let collinear: Vec<CollinearCondition> = figure
            .conditions()
            .iter()
            .filter_map(|c| match c {
                Condition::Collinear(c) => Some(*c),
                _ => None,
            })
            .collect();

        let mut candidates = Vec::new();
        for i in 0..parallels.len() {
            for j in (i + 1)..parallels.len() {
                if let Some((a, b, c, d)) = parallelogram(&parallels[i], &parallels[j], &collinear)
                {
                    let fathers = vec![
                        Condition::Parallel(parallels[i].clone()),
                        Condition::Parallel(parallels[j].clone()),
                    ];
                    push_parts(&mut candidates, a, b, c, d, &fathers);
                }
            }
        }

        let mut changed = false;
        for (condition, fathers) in candidates {
            changed |= figure.add_derived(condition, fathers, TransferType::Parallelogram);
        }
        changed
    }
}

type Quad = (Point, Point, Point, Point);

// Two parallel pairs form the quadrilateral a,b,c,d (in order) when the
// lines share endpoints as sides: ab with cd, and bc with da. Known
// collinear triples inside the quadrilateral exclude degenerate cases.
fn parallelogram(
    p: &ParallelCondition,
    q: &ParallelCondition,
    collinear: &[CollinearCondition],
) -> Option<Quad> {
    let first = [line_pts(p.a), line_pts(p.b)];
    let second = [line_pts(q.a), line_pts(q.b)];
    for x in 0..2 {
        for f1 in [false, true] {
            for f2 in [false, true] {
                let side1 = or_swap(first[x], f1);
                let side2 = or_swap(first[1 - x], f2);
                for y in 0..2 {
                    for f3 in [false, true] {
                        for f4 in [false, true] {
                            let side3 = or_swap(second[y], f3);
                            let side4 = or_swap(second[1 - y], f4);
                            // sides in order: side1(a,b), side3(b,c), side2(c,d), side4(d,a)
                            if side1.1 == side3.0
                                && side3.1 == side2.0
                                && side2.1 == side4.0
                                && side4.1 == side1.0
                            {
                                let (a, b) = side1;
                                let c = side3.1;
                                let d = side2.1;
                                let pts = [a, b, c, d];
                                if new_points(&pts) && !degenerate(&pts, collinear) {
                                    return Some((a, b, c, d));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

fn or_swap(l: (Point, Point), swap_: bool) -> (Point, Point) {
    if swap_ { (l.1, l.0) } else { l }
}

fn new_points(pts: &[Point; 4]) -> bool {
    for i in 0..4 {
        for j in (i + 1)..4 {
            if pts[i] == pts[j] {
                return false;
            }
        }
    }
    true
}

// A known collinear triple among the vertices rules the quadrilateral out.
fn degenerate(pts: &[Point; 4], collinear: &[CollinearCondition]) -> bool {
    collinear.iter().any(|c| {
        let known = [c.a, c.b, c.c];
        known.iter().all(|p| pts.contains(p))
    })
}

fn line_pts(l: PanLine) -> (Point, Point) {
    match l {
        PanLine::Line(l) => (l.a, l.b),
        PanLine::Segment(s) => (s.a, s.b),
        PanLine::Ray(r) => (r.origin, r.through),
    }
}

fn push_parts(
    candidates: &mut Vec<(Condition, Vec<Condition>)>,
    a: Point,
    b: Point,
    c: Point,
    d: Point,
    fathers: &[Condition],
) {
    let fathers = fathers.to_vec();
    let ab_cd = Condition::Sum(SumCondition::new(
        vec![len_object(a, b)],
        vec![len_object(c, d)],
        Ratio::from(0),
    ));
    let bc_da = Condition::Sum(SumCondition::new(
        vec![len_object(b, c)],
        vec![len_object(d, a)],
        Ratio::from(0),
    ));
    let ang_a_c = Condition::Sum(SumCondition::new(
        vec![ang_object(d, a, b)],
        vec![ang_object(b, c, d)],
        Ratio::from(0),
    ));
    let ang_b_d = Condition::Sum(SumCondition::new(
        vec![ang_object(a, b, c)],
        vec![ang_object(c, d, a)],
        Ratio::from(0),
    ));
    for condition in [ab_cd, bc_da, ang_a_c, ang_b_d] {
        candidates.push((condition, fathers.clone()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::figure::Object;
    use crate::prove::solve::{Rule, solve};

    #[test]
    fn derives_opposite_side_equality() {
        let mut figure = Figure::new();
        let p: Vec<_> = (0..4).map(crate::figure::Point::new).collect();
        figure.add_condition(Condition::Parallel(ParallelCondition::new(
            PanLine::line(p[0], p[1]),
            PanLine::line(p[3], p[2]),
        )));
        figure.add_condition(Condition::Parallel(ParallelCondition::new(
            PanLine::line(p[1], p[2]),
            PanLine::line(p[0], p[3]),
        )));
        let target = Condition::Sum(SumCondition::new(
            vec![Object::length(p[0], p[1])],
            vec![Object::length(p[2], p[3])],
            Ratio::from(0),
        ));
        let proof = solve(&mut figure, &target, &Rule::new()).expect("parallelogram");
        assert!(proof.describe.contains(&TransferType::Parallelogram));
    }
}
