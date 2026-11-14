use crate::figure::Figure;
use crate::prove::condition::Condition;
use crate::prove::proof::Proof;
use crate::prove::transfer::base_transfer::BaseTransfer;
use crate::prove::transfer::congruent_transfer::{CongruentTransfer, Criterion};
use crate::prove::transfer::parallel_angle_transfer::ParallelAngleTransfer;
use crate::prove::transfer::parallel_transfer::ParallelTransfer;

pub struct Rule {
    pub parallel_transitive: bool,
    pub parallel_corresponding: bool,
    pub parallel_alternate_interior: bool,
    pub parallel_co_interior: bool,
    pub congruent_sss: bool,
    pub congruent_sas: bool,
    pub congruent_asa: bool,
    pub congruent_aas: bool,
    pub congruent_hl: bool,
}

impl Rule {
    pub fn new() -> Self {
        Rule {
            parallel_transitive: true,
            parallel_corresponding: true,
            parallel_alternate_interior: true,
            parallel_co_interior: true,
            congruent_sss: true,
            congruent_sas: true,
            congruent_asa: true,
            congruent_aas: true,
            congruent_hl: true,
        }
    }

    pub fn run(&self, figure: &mut Figure) -> bool {
        let mut changed = false;
        if self.parallel_transitive {
            changed |= ParallelTransfer.transfer(figure);
        }
        if self.parallel_corresponding || self.parallel_alternate_interior || self.parallel_co_interior {
            changed |= ParallelAngleTransfer {
                corresponding: self.parallel_corresponding,
                alternate_interior: self.parallel_alternate_interior,
                co_interior: self.parallel_co_interior,
            }
            .transfer(figure);
        }
        let mut criteria = Vec::new();
        if self.congruent_sss {
            criteria.push(Criterion::Sss);
        }
        if self.congruent_sas {
            criteria.push(Criterion::Sas);
        }
        if self.congruent_asa {
            criteria.push(Criterion::Asa);
        }
        if self.congruent_aas {
            criteria.push(Criterion::Aas);
        }
        if self.congruent_hl {
            criteria.push(Criterion::Hl);
        }
        if !criteria.is_empty() {
            changed |= CongruentTransfer { criteria }.transfer(figure);
        }
        changed
    }
}

impl Default for Rule {
    fn default() -> Self {
        Rule::new()
    }
}

pub fn solve(figure: &mut Figure, target: &Condition, rule: &Rule) -> Option<Proof> {
    loop {
        if let Some(proof) = figure.proof_of(target) {
            return Some(proof.clone());
        }
        if !rule.run(figure) {
            return None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Rule, solve};
    use crate::figure::{Figure, Object, PanLine, Point, Segment};
    use crate::prove::condition::{
        CollinearCondition, Condition, ParallelCondition, SideCondition, SumCondition,
    };
    use crate::prove::transfer::TransferType;
    use crate::ratio::Ratio;

    #[test]
    fn solves_transitive_parallel() {
        let x = PanLine::Segment(Segment::new(Point::new(0), Point::new(1)));
        let y = PanLine::Segment(Segment::new(Point::new(2), Point::new(3)));
        let z = PanLine::Segment(Segment::new(Point::new(4), Point::new(5)));

        let mut figure = Figure::new();
        figure.add_condition(Condition::Parallel(ParallelCondition::new(x, y)));
        figure.add_condition(Condition::Parallel(ParallelCondition::new(y, z)));

        let target = Condition::Parallel(ParallelCondition::new(x, z));
        let proof = solve(&mut figure, &target, &Rule::new()).expect("target should be proven");
        assert_eq!(proof.describe, vec![TransferType::ParallelTransitive]);
    }

    // Two parallel lines ab ∥ cd cut by the transversal bc; a and d are the
    // other points of the two lines, so the angles at b and c are the interior
    // angles formed by the transversal.
    fn parallel_figure(figure: &mut Figure, same: bool) -> (Point, Point, Point, Point) {
        let a = figure.add_point();
        let b = figure.add_point();
        let c = figure.add_point();
        let d = figure.add_point();
        let l1 = PanLine::line(a, b);
        let l2 = PanLine::line(c, d);
        let t = PanLine::line(b, c);
        figure.add_condition(Condition::Parallel(ParallelCondition::new(l1, l2)));
        figure.add_condition(Condition::Side(SideCondition::new(t, a, d, same)));
        (a, b, c, d)
    }

    #[test]
    fn solves_alternate_interior_angles() {
        let mut figure = Figure::new();
        let (a, b, c, d) = parallel_figure(&mut figure, false);
        let target = Condition::Sum(SumCondition::new(
            vec![Object::angle(a, b, c)],
            vec![Object::angle(d, c, b)],
            Ratio::from(0),
        ));
        let proof = solve(&mut figure, &target, &Rule::new()).expect("alternate interior angles");
        assert_eq!(proof.describe, vec![TransferType::ParallelAlternateInterior]);
    }

    #[test]
    fn solves_co_interior_angles() {
        let mut figure = Figure::new();
        let (a, b, c, d) = parallel_figure(&mut figure, true);
        let target = Condition::Sum(SumCondition::new(
            vec![Object::angle(a, b, c), Object::angle(d, c, b)],
            vec![],
            Ratio::from(180),
        ));
        let proof = solve(&mut figure, &target, &Rule::new()).expect("co-interior angles");
        assert_eq!(proof.describe, vec![TransferType::ParallelCoInterior]);
    }

    #[test]
    fn solves_corresponding_angles() {
        let mut figure = Figure::new();
        let (a, b, c, d) = parallel_figure(&mut figure, true);
        let e = figure.add_point();
        let l2 = PanLine::line(c, d);
        // e lies on the transversal beyond c, on the far side of cd from b.
        figure.add_condition(Condition::Collinear(CollinearCondition::new(b, c, e)));
        figure.add_condition(Condition::Side(SideCondition::new(l2, e, b, false)));
        let target = Condition::Sum(SumCondition::new(
            vec![Object::angle(a, b, c)],
            vec![Object::angle(d, c, e)],
            Ratio::from(0),
        ));
        let proof = solve(&mut figure, &target, &Rule::new()).expect("corresponding angles");
        assert_eq!(proof.describe, vec![TransferType::ParallelCorresponding]);
    }
}
