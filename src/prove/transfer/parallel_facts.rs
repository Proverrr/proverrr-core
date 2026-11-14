use std::collections::{BTreeSet, HashMap};

use crate::figure::{Figure, PanLine, Point};
use crate::prove::condition::{CollinearCondition, Condition, ParallelCondition, SideCondition};

pub type SideKey = ((Point, Point), (Point, Point));
pub type SideMap = HashMap<SideKey, SideCondition>;
pub type CollinearSet = BTreeSet<(Point, Point, Point)>;

pub fn pair(a: Point, b: Point) -> (Point, Point) {
    if a <= b { (a, b) } else { (b, a) }
}

pub fn line_points(line: PanLine) -> [Point; 2] {
    match line {
        PanLine::Line(l) => [l.a, l.b],
        PanLine::Segment(s) => [s.a, s.b],
        PanLine::Ray(r) => [r.origin, r.through],
    }
}

// A line as an unordered pair of points, so a line, segment or ray through the
// same two points all give the same key.
pub fn line_key(line: PanLine) -> (Point, Point) {
    let [a, b] = line_points(line);
    pair(a, b)
}

pub fn collect_parallel(figure: &Figure) -> Vec<ParallelCondition> {
    figure
        .conditions()
        .iter()
        .filter_map(|cond| match cond {
            Condition::Parallel(p) => Some(p.clone()),
            _ => None,
        })
        .collect()
}

pub fn collect_sides(figure: &Figure) -> SideMap {
    let mut map = HashMap::new();
    for cond in figure.conditions() {
        if let Condition::Side(s) = cond {
            map.insert((line_key(s.line), pair(s.a, s.b)), *s);
        }
    }
    map
}

pub fn collect_collinear(figure: &Figure) -> CollinearSet {
    let mut set = BTreeSet::new();
    for cond in figure.conditions() {
        if let Condition::Collinear(c) = cond {
            set.insert((c.a, c.b, c.c));
        }
    }
    set
}

// A transversal cutting the two parallel lines: it meets line a at p and line
// b at q; x and y are the other points of the two lines, and same tells which
// side of the transversal they lie on.
pub struct Config {
    pub p: Point,
    pub q: Point,
    pub x: Point,
    pub y: Point,
    pub same: bool,
    pub side: SideCondition,
}

pub fn match_config(parallel: &ParallelCondition, side: &SideCondition) -> Option<Config> {
    let l1 = line_points(parallel.a);
    let l2 = line_points(parallel.b);
    let t = line_points(side.line);
    for (x, y) in [(side.a, side.b), (side.b, side.a)] {
        if !l1.contains(&x) || !l2.contains(&y) {
            continue;
        }
        let p = if l1[0] == x { l1[1] } else { l1[0] };
        let q = if l2[0] == y { l2[1] } else { l2[0] };
        if p == q || pair(p, q) != pair(t[0], t[1]) {
            continue;
        }
        return Some(Config { p, q, x, y, same: side.same, side: *side });
    }
    None
}

// Whether the transversal point t_point lies on the interior ray at the vertex,
// i.e. on the same side of the parallel line as the other intersection. The
// other intersection itself is always interior. Returns the side condition used.
pub fn interior_info(
    t_point: Point,
    other: Point,
    line: PanLine,
    sides: &SideMap,
) -> Option<(bool, Option<SideCondition>)> {
    if t_point == other {
        return Some((true, None));
    }
    let s = sides.get(&(line_key(line), pair(t_point, other)))?;
    Some((s.same, Some(*s)))
}

pub fn points_on_line(p: Point, q: Point, collinear: &CollinearSet) -> Vec<Point> {
    let mut points = vec![p, q];
    for &(a, b, c) in collinear {
        let triple = [a, b, c];
        if triple.contains(&p) && triple.contains(&q) {
            for z in triple {
                if z != p && z != q && !points.contains(&z) {
                    points.push(z);
                }
            }
        }
    }
    points
}

pub fn collinear_father(
    p: Point,
    q: Point,
    z: Point,
    collinear: &CollinearSet,
) -> Option<Condition> {
    let c = CollinearCondition::new(p, q, z);
    if collinear.contains(&(c.a, c.b, c.c)) {
        Some(Condition::Collinear(c))
    } else {
        None
    }
}