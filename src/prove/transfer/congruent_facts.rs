use std::collections::{BTreeSet, HashMap};

use crate::figure::{Figure, Point, ObjectType};
use crate::prove::condition::Condition;
use crate::ratio::Ratio;

pub type LenKey = (Point, Point);
pub type AngleKey = (Point, Point, Point);

pub fn len_key(p: Point, q: Point) -> LenKey {
    if p <= q { (p, q) } else { (q, p) }
}

// Vertex first, then the two rays in a fixed order.
pub fn ang_key(a: Point, b: Point, c: Point) -> AngleKey {
    let (lo, hi) = if a <= c { (a, c) } else { (c, a) };
    (b, lo, hi)
}

// The equalities and right angles read from the figure's Sum conditions.
pub struct Facts {
    length_eq: HashMap<(LenKey, LenKey), Condition>,
    angle_eq: HashMap<(AngleKey, AngleKey), Condition>,
    right: HashMap<AngleKey, Condition>,
    points: BTreeSet<Point>,
}

impl Facts {
    pub fn from_figure(figure: &Figure) -> Self {
        let mut facts = Facts {
            length_eq: HashMap::new(),
            angle_eq: HashMap::new(),
            right: HashMap::new(),
            points: BTreeSet::new(),
        };
        for cond in figure.conditions() {
            let Condition::Sum(s) = cond else { continue };
            if s.a.len() == 1 && s.b_objects.len() == 1 && s.b_const.is_zero() {
                let x = &s.a[0];
                let y = &s.b_objects[0];
                match (x.kind, y.kind) {
                    (ObjectType::Length, ObjectType::Length) => {
                        let k1 = len_key(x.points[0], x.points[1]);
                        let k2 = len_key(y.points[0], y.points[1]);
                        facts.length_eq.insert((k1, k2), cond.clone());
                        facts.length_eq.insert((k2, k1), cond.clone());
                    }
                    (ObjectType::Angle, ObjectType::Angle) => {
                        let k1 = ang_key(x.points[0], x.points[1], x.points[2]);
                        let k2 = ang_key(y.points[0], y.points[1], y.points[2]);
                        facts.angle_eq.insert((k1, k2), cond.clone());
                        facts.angle_eq.insert((k2, k1), cond.clone());
                    }
                    _ => {}
                }
                for p in x.points.iter().chain(y.points.iter()) {
                    facts.points.insert(*p);
                }
            } else if s.a.len() == 1 && s.b_objects.is_empty() && s.b_const == Ratio::from(90) {
                let x = &s.a[0];
                if x.kind == ObjectType::Angle {
                    let k = ang_key(x.points[0], x.points[1], x.points[2]);
                    facts.right.insert(k, cond.clone());
                    for p in &x.points {
                        facts.points.insert(*p);
                    }
                }
            }
        }
        facts
    }

    pub fn triangles(&self) -> Vec<[Point; 3]> {
        let pts: Vec<Point> = self.points.iter().copied().collect();
        let mut out = Vec::new();
        for i in 0..pts.len() {
            for j in (i + 1)..pts.len() {
                for k in (j + 1)..pts.len() {
                    out.push([pts[i], pts[j], pts[k]]);
                }
            }
        }
        out
    }

    pub fn length_known(&self, k1: LenKey, k2: LenKey) -> bool {
        k1 == k2 || self.length_eq.contains_key(&(k1, k2))
    }

    pub fn any_side_equal(&self, a: [Point; 3], b: [Point; 3]) -> bool {
        let sa = [[a[0], a[1]], [a[1], a[2]], [a[2], a[0]]];
        let sb = [[b[0], b[1]], [b[1], b[2]], [b[2], b[0]]];
        for x in sa {
            for y in sb {
                if self.length_known(len_key(x[0], x[1]), len_key(y[0], y[1])) {
                    return true;
                }
            }
        }
        false
    }

    // A shared side is equal to itself and needs no father.
    pub fn side_equal(&self, a: Point, b: Point, c: Point, d: Point, fathers: &mut Vec<Condition>) -> bool {
        let k1 = len_key(a, b);
        let k2 = len_key(c, d);
        if k1 == k2 {
            return true;
        }
        match self.length_eq.get(&(k1, k2)) {
            Some(cond) => {
                fathers.push(cond.clone());
                true
            }
            None => false,
        }
    }

    // Two right angles are equal even without a direct equality condition.
    pub fn angle_equal(&self, x: AngleKey, y: AngleKey, fathers: &mut Vec<Condition>) -> bool {
        let k1 = ang_key(x.0, x.1, x.2);
        let k2 = ang_key(y.0, y.1, y.2);
        if k1 == k2 {
            return true;
        }
        if let Some(cond) = self.angle_eq.get(&(k1, k2)) {
            fathers.push(cond.clone());
            return true;
        }
        if let (Some(c1), Some(c2)) = (self.right.get(&k1), self.right.get(&k2)) {
            fathers.push(c1.clone());
            fathers.push(c2.clone());
            return true;
        }
        false
    }

    pub fn right_equal(&self, x: AngleKey, y: AngleKey, fathers: &mut Vec<Condition>) -> bool {
        let k1 = ang_key(x.0, x.1, x.2);
        let k2 = ang_key(y.0, y.1, y.2);
        match (self.right.get(&k1), self.right.get(&k2)) {
            (Some(c1), Some(c2)) => {
                fathers.push(c1.clone());
                fathers.push(c2.clone());
                true
            }
            _ => false,
        }
    }
}