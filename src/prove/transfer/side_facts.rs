use crate::figure::{Figure, PanLine, Point};
use crate::prove::condition::{BetweenCondition, Condition, SideCondition};
use crate::prove::transfer::TransferType;
use crate::prove::transfer::base_transfer::BaseTransfer;

pub struct SideTransfer;

// Chains side relations on one line, and turns between-ness at a point of a
// line into opposite sides of that line. Points are assumed off the line.
impl BaseTransfer for SideTransfer {
    fn transfer(&self, figure: &mut Figure) -> bool {
        let sides: Vec<SideCondition> = figure
            .conditions()
            .iter()
            .filter_map(|c| match c {
                Condition::Side(s) => Some(*s),
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
        let lines = known_lines(figure);

        let mut candidates = Vec::new();
        for i in 0..sides.len() {
            for j in (i + 1)..sides.len() {
                if sides[i].line != sides[j].line {
                    continue;
                }
                if let Some((x, y, same)) = combine(&sides[i], &sides[j]) {
                    let cond = Condition::Side(SideCondition::new(sides[i].line, x, y, same));
                    candidates.push((
                        cond,
                        vec![Condition::Side(sides[i]), Condition::Side(sides[j])],
                    ));
                }
            }
        }
        for b in &betweens {
            for line in &lines {
                let (a, c) = line_points(*line);
                let on_line = |p: Point| p == a || p == c || collinear_with(figure, a, c, p);
                if !on_line(b.mid) || on_line(b.a) || on_line(b.b) {
                    continue;
                }
                let cond = Condition::Side(SideCondition::new(*line, b.a, b.b, false));
                let mut fathers = vec![Condition::Between(*b)];
                if let Some(c) = collinear_father(figure, a, c, b.mid) {
                    fathers.push(c);
                }
                candidates.push((cond, fathers));
            }
        }

        let mut changed = false;
        for (condition, fathers) in candidates {
            changed |= figure.add_derived(condition, fathers, TransferType::SideFacts);
        }
        changed
    }
}

// Two side relations on one line sharing exactly one point give the relation
// between the other two points.
fn combine(x: &SideCondition, y: &SideCondition) -> Option<(Point, Point, bool)> {
    let shared = |p: Point, q: Point| p == q;
    for (a1, b1, a2, b2) in [
        (x.a, x.b, y.a, y.b),
        (x.a, x.b, y.b, y.a),
        (x.b, x.a, y.a, y.b),
        (x.b, x.a, y.b, y.a),
    ] {
        if shared(a1, a2) && b1 != b2 {
            return Some((b1, b2, x.same == y.same));
        }
    }
    None
}

fn known_lines(figure: &Figure) -> Vec<PanLine> {
    let mut lines = Vec::new();
    for cond in figure.conditions() {
        match cond {
            Condition::Parallel(p) => {
                for l in [p.a, p.b] {
                    if !lines.contains(&l) {
                        lines.push(l);
                    }
                }
            }
            Condition::Perpendicular(p) => {
                for l in [p.a, p.b] {
                    if !lines.contains(&l) {
                        lines.push(l);
                    }
                }
            }
            Condition::Side(s) if !lines.contains(&s.line) => lines.push(s.line),
            _ => {}
        }
    }
    lines
}

fn line_points(l: PanLine) -> (Point, Point) {
    match l {
        PanLine::Line(l) => (l.a, l.b),
        PanLine::Segment(s) => (s.a, s.b),
        PanLine::Ray(r) => (r.origin, r.through),
    }
}

fn collinear_with(figure: &Figure, a: Point, c: Point, p: Point) -> bool {
    figure.conditions().iter().any(|cond| {
        matches!(cond, Condition::Collinear(c_)
            if [c_.a, c_.b, c_.c].contains(&a) && [c_.a, c_.b, c_.c].contains(&c) && [c_.a, c_.b, c_.c].contains(&p))
    })
}

fn collinear_father(figure: &Figure, a: Point, c: Point, p: Point) -> Option<Condition> {
    figure.conditions().iter().find_map(|cond| match cond {
        Condition::Collinear(c_)
            if [c_.a, c_.b, c_.c].contains(&a)
                && [c_.a, c_.b, c_.c].contains(&c)
                && [c_.a, c_.b, c_.c].contains(&p) =>
        {
            Some(cond.clone())
        }
        _ => None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::figure::Point;

    #[test]
    fn chains_side_relations() {
        let mut figure = Figure::new();
        let x = Point::new(0);
        let y = Point::new(1);
        let z = Point::new(2);
        let l = PanLine::line(Point::new(3), Point::new(4));
        figure.add_condition(Condition::Side(SideCondition::new(l, x, y, true)));
        figure.add_condition(Condition::Side(SideCondition::new(l, y, z, true)));

        let target = Condition::Side(SideCondition::new(l, x, z, true));
        let proof =
            crate::prove::solve::solve(&mut figure, &target, &crate::prove::solve::Rule::new())
                .expect("side chain");
        assert!(proof.describe.contains(&TransferType::SideFacts));
    }

    // Between-ness at a point of the line gives opposite sides.
    #[test]
    fn between_gives_opposite_sides() {
        let mut figure = Figure::new();
        let l0 = figure.add_point();
        let l1 = figure.add_point();
        let a = figure.add_point();
        let b = figure.add_point();
        // l1 lies between a and b, and l1 is an endpoint of the known line.
        figure.add_condition(Condition::Between(BetweenCondition::new(a, l1, b)));
        figure.add_condition(Condition::Parallel(
            crate::prove::condition::ParallelCondition::new(
                PanLine::line(l0, l1),
                PanLine::line(a, b),
            ),
        ));

        let target = Condition::Side(SideCondition::new(PanLine::line(l0, l1), a, b, false));
        let proof =
            crate::prove::solve::solve(&mut figure, &target, &crate::prove::solve::Rule::new())
                .expect("opposite sides");
        assert!(proof.describe.contains(&TransferType::SideFacts));
    }
}
