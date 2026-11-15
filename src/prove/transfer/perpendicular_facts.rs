use crate::figure::{Figure, PanLine};
use crate::prove::condition::{CollinearCondition, Condition, ParallelCondition, PerpendicularCondition, SumCondition};
use crate::prove::transfer::TransferType;
use crate::prove::transfer::base_transfer::BaseTransfer;
use crate::prove::transfer::congruent_facts::ang_object;
use crate::ratio::Ratio;

pub struct PerpendicularTransfer;

// A right angle sits where two perpendicular lines meet, and a line
// perpendicular to one of two parallels is perpendicular to the other.
impl BaseTransfer for PerpendicularTransfer {
    fn transfer(&self, figure: &mut Figure) -> bool {
        let perps: Vec<PerpendicularCondition> = figure
            .conditions()
            .iter()
            .filter_map(|c| match c {
                Condition::Perpendicular(p) => Some(*p),
                _ => None,
            })
            .collect();
        let parallels: Vec<ParallelCondition> = figure
            .conditions()
            .iter()
            .filter_map(|c| match c {
                Condition::Parallel(p) => Some(p.clone()),
                _ => None,
            })
            .collect();

        let mut candidates: Vec<(Condition, Vec<Condition>)> = Vec::new();
        for p in &perps {
            for m in common_points(p) {
                let xs = points_on_line(figure, p.a, m);
                let ys = points_on_line(figure, p.b, m);
                for (x, fx) in &xs {
                    for (y, fy) in &ys {
                        let condition = Condition::Sum(SumCondition::new(
                            vec![ang_object(*x, m, *y)],
                            vec![],
                            Ratio::from(90),
                        ));
                        let mut fathers = vec![Condition::Perpendicular(*p)];
                        if let Some(c) = fx {
                            fathers.push(Condition::Collinear(*c));
                        }
                        if let Some(c) = fy {
                            fathers.push(Condition::Collinear(*c));
                        }
                        candidates.push((condition, fathers));
                    }
                }
            }
        }
        for p in &parallels {
            for q in &perps {
                if let Some((x, y)) = perp_between(p, q) {
                    let condition = Condition::Perpendicular(PerpendicularCondition::new(x, y));
                    candidates.push((
                        condition,
                        vec![Condition::Parallel(p.clone()), Condition::Perpendicular(*q)],
                    ));
                }
            }
        }

        let mut changed = false;
        for (condition, fathers) in candidates {
            changed |= figure.add_derived(condition, fathers, TransferType::PerpendicularFacts);
        }
        changed
    }
}

fn common_points(p: &PerpendicularCondition) -> Vec<crate::figure::Point> {
    let (a1, b1) = line_points(p.a);
    let (a2, b2) = line_points(p.b);
    let mut out = Vec::new();
    for x in [a1, b1] {
        if x == a2 || x == b2 {
            out.push(x);
        }
    }
    out
}

fn line_points(l: PanLine) -> (crate::figure::Point, crate::figure::Point) {
    match l {
        PanLine::Line(l) => (l.a, l.b),
        PanLine::Segment(s) => (s.a, s.b),
        PanLine::Ray(r) => (r.origin, r.through),
    }
}

fn points_on_line(
    figure: &Figure,
    line: PanLine,
    m: crate::figure::Point,
) -> Vec<(crate::figure::Point, Option<CollinearCondition>)> {
    let (a, b) = line_points(line);
    let mut out: Vec<(crate::figure::Point, Option<CollinearCondition>)> =
        vec![(a, None), (b, None)];
    for cond in figure.conditions() {
        if let Condition::Collinear(c @ CollinearCondition { a: x, b: y, c: z }) = cond {
            let pts = [*x, *y, *z];
            if pts.contains(&a) && pts.contains(&b) {
                for p in pts {
                    if p != m && !out.iter().any(|(q, _)| *q == p) {
                        out.push((p, Some(*c)));
                    }
                }
            }
        }
    }
    out.retain(|(p, _)| *p != m);
    out
}

// Parallel(a, b) and Perpendicular(c, b): a and c are both perpendicular to
// b, so the perpendicular pair to derive is (a, c).
fn perp_between(
    p: &ParallelCondition,
    q: &PerpendicularCondition,
) -> Option<(PanLine, PanLine)> {
    if p.b == q.b {
        return Some((p.a, q.a));
    }
    if p.a == q.b {
        return Some((p.b, q.a));
    }
    if p.b == q.a {
        return Some((p.a, q.b));
    }
    if p.a == q.a {
        return Some((p.b, q.b));
    }
    None
}
