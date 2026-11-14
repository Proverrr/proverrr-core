//! Geometric elements and the conditions known about them.

/// A full line through two points.
pub mod line;
/// Measurable quantities such as lengths and angles.
pub mod object;
/// Abstract points identified by their index in a figure.
pub mod point;
/// Half a line: an origin and a direction.
pub mod ray;
/// The part of a line between two points.
pub mod segment;
/// A triangle defined by three vertices.
pub mod triangle;

pub use line::Line;
pub use object::{Object, ObjectType};
pub use point::Point;
pub use ray::Ray;
pub use segment::Segment;
pub use triangle::Triangle;

use crate::prove::condition::Condition;
use crate::prove::proof::Proof;
use crate::prove::transfer::TransferType;

/// Index of an element inside a [`Figure`].
///
/// IDs are handed out by the `add_*` methods and are only meaningful for the
/// figure that returned them.
pub type ID = u32;

/// A line, a segment or a ray.
///
/// Conditions such as parallelism accept all three, so they share one type.
/// The variants are ordered, and conditions rely on that order to normalize
/// their arguments.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PanLine {
    Line(Line),
    Segment(Segment),
    Ray(Ray),
}

impl PanLine {
    /// The line through the points `a` and `b`.
    pub fn line(a: Point, b: Point) -> Self {
        PanLine::Line(Line::new(a, b))
    }

    /// The segment between the points `a` and `b`.
    pub fn segment(a: Point, b: Point) -> Self {
        PanLine::Segment(Segment::new(a, b))
    }

    /// The ray from `origin` through `through`.
    pub fn ray(origin: Point, through: Point) -> Self {
        PanLine::Ray(Ray::new(origin, through))
    }
}

/// All elements of one geometry problem and the conditions known about them.
///
/// Elements are added with the `add_*` methods, which return an [`ID`]; they are
/// read back by that ID or through the slice accessors. A condition is either
/// assumed with [`Figure::add_condition`] or derived by [`Figure::add_derived`],
/// and always comes with a [`Proof`].
///
/// # Example
/// ```
/// use proverrr_core::figure::{Figure, Segment};
///
/// let mut figure = Figure::new();
/// let a = figure.add_point();
/// let b = figure.add_point();
/// let c = figure.add_point();
///
/// let bottom = figure.add_segment(Segment::new(a, b));
/// figure.add_segment(Segment::new(a, c));
///
/// assert_eq!(bottom, 0);
/// assert_eq!(figure.point(a.id()), Some(&a));
/// assert_eq!(figure.segment(bottom), Some(&Segment::new(a, b)));
/// assert_eq!(figure.points().len(), 3);
/// ```
#[derive(Default)]
pub struct Figure {
    points: Vec<Point>,
    lines: Vec<Line>,
    segments: Vec<Segment>,
    rays: Vec<Ray>,
    objects: Vec<Object>,
    conditions: Vec<Condition>,
    proofs: Vec<Proof>,
}

impl Figure {
    /// Creates a figure with no elements and no known conditions.
    pub fn new() -> Self {
        Figure::default()
    }

    /// Adds a new point and returns it.
    pub fn add_point(&mut self) -> Point {
        let point = Point::new(self.points.len() as ID);
        self.points.push(point);
        point
    }

    /// Adds `line` and returns its ID.
    ///
    /// The line refers to existing points by ID.
    pub fn add_line(&mut self, line: Line) -> ID {
        self.lines.push(line);
        (self.lines.len() - 1) as ID
    }

    /// Adds `segment` and returns its ID.
    ///
    /// The segment refers to existing points by ID.
    pub fn add_segment(&mut self, segment: Segment) -> ID {
        self.segments.push(segment);
        (self.segments.len() - 1) as ID
    }

    /// Adds `ray` and returns its ID.
    ///
    /// The ray refers to existing points by ID.
    pub fn add_ray(&mut self, ray: Ray) -> ID {
        self.rays.push(ray);
        (self.rays.len() - 1) as ID
    }

    /// Adds `object` and returns its ID.
    ///
    /// The object is a measurable quantity such as a length or an angle; it
    /// refers to existing points by ID.
    pub fn add_object(&mut self, object: Object) -> ID {
        self.objects.push(object);
        (self.objects.len() - 1) as ID
    }

    /// Assumes `condition` and returns its ID.
    ///
    /// An assumed condition is stored with an empty proof, because nothing
    /// derived it.
    pub fn add_condition(&mut self, condition: Condition) -> ID {
        self.conditions.push(condition);
        self.proofs.push(Proof::new());
        (self.conditions.len() - 1) as ID
    }

    /// Returns the point with the given ID, or `None` if there is none.
    pub fn point(&self, id: ID) -> Option<&Point> {
        self.points.get(id as usize)
    }

    /// Returns the line with the given ID, or `None` if there is none.
    pub fn line(&self, id: ID) -> Option<&Line> {
        self.lines.get(id as usize)
    }

    /// Returns the segment with the given ID, or `None` if there is none.
    pub fn segment(&self, id: ID) -> Option<&Segment> {
        self.segments.get(id as usize)
    }

    /// Returns the ray with the given ID, or `None` if there is none.
    pub fn ray(&self, id: ID) -> Option<&Ray> {
        self.rays.get(id as usize)
    }

    /// Returns the object with the given ID, or `None` if there is none.
    pub fn object(&self, id: ID) -> Option<&Object> {
        self.objects.get(id as usize)
    }

    /// Returns the condition with the given ID, or `None` if there is none.
    pub fn condition(&self, id: ID) -> Option<&Condition> {
        self.conditions.get(id as usize)
    }

    /// Returns all points, in the order they were added.
    pub fn points(&self) -> &[Point] {
        &self.points
    }

    /// Returns all lines, in the order they were added.
    pub fn lines(&self) -> &[Line] {
        &self.lines
    }

    /// Returns all segments, in the order they were added.
    pub fn segments(&self) -> &[Segment] {
        &self.segments
    }

    /// Returns all rays, in the order they were added.
    pub fn rays(&self) -> &[Ray] {
        &self.rays
    }

    /// Returns all objects, in the order they were added.
    pub fn objects(&self) -> &[Object] {
        &self.objects
    }

    /// Returns all known conditions, in the order they were added.
    pub fn conditions(&self) -> &[Condition] {
        &self.conditions
    }

    /// Returns the proof of every known condition.
    ///
    /// `proofs()[i]` belongs to `conditions()[i]`.
    pub fn proofs(&self) -> &[Proof] {
        &self.proofs
    }

    /// Returns the position of `condition` among the known conditions.
    pub fn index_of(&self, condition: &Condition) -> Option<usize> {
        self.conditions.iter().position(|known| known == condition)
    }

    /// Returns the proof of `condition`, or `None` if the figure does not know it.
    ///
    /// An assumed condition has an empty proof; a derived one carries the steps
    /// that led to it.
    pub fn proof_of(&self, condition: &Condition) -> Option<&Proof> {
        self.index_of(condition).map(|index| &self.proofs[index])
    }

    /// Adds `condition` as derived from `fathers` by one `describe` step.
    ///
    /// Returns `false` and changes nothing if `condition` is already known.
    /// Fathers unknown to the figure are skipped.
    pub fn add_derived(
        &mut self,
        condition: Condition,
        fathers: Vec<Condition>,
        describe: TransferType,
    ) -> bool {
        if self.index_of(&condition).is_some() {
            return false;
        }
        let mut proof = Proof::new();
        for father in &fathers {
            if let Some(father_proof) = self.proof_of(father) {
                proof = proof + father_proof.clone();
            }
        }
        proof.push_step(condition.clone(), fathers, describe);
        self.conditions.push(condition);
        self.proofs.push(proof);
        true
    }
}
