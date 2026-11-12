pub mod line;
pub mod object;
pub mod point;
pub mod ray;
pub mod segment;

pub use line::Line;
pub use object::{Object, ObjectType};
pub use point::Point;
pub use ray::Ray;
pub use segment::Segment;

use crate::prove::condition::Condition;
use crate::prove::proof::Proof;
use crate::prove::transfer::TransferType;

pub type ID = u32;

// Line, Segment and Ray are all subsets of a line.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PanLine {
    Line(Line),
    Segment(Segment),
    Ray(Ray),
}

#[derive(Default)]
pub struct Figure {
    pub points: Vec<Point>,
    pub lines: Vec<Line>,
    pub segments: Vec<Segment>,
    pub rays: Vec<Ray>,
    pub objects: Vec<Object>,
    pub conditions: Vec<Condition>,
    pub proofs: Vec<Proof>,
}

impl Figure {
    pub fn new() -> Self {
        Figure::default()
    }

    pub fn add_point(&mut self, point: Point) -> ID {
        self.points.push(point);
        (self.points.len() - 1) as ID
    }

    pub fn add_line(&mut self, line: Line) -> ID {
        self.lines.push(line);
        (self.lines.len() - 1) as ID
    }

    pub fn add_segment(&mut self, segment: Segment) -> ID {
        self.segments.push(segment);
        (self.segments.len() - 1) as ID
    }

    pub fn add_ray(&mut self, ray: Ray) -> ID {
        self.rays.push(ray);
        (self.rays.len() - 1) as ID
    }

    pub fn add_object(&mut self, object: Object) -> ID {
        self.objects.push(object);
        (self.objects.len() - 1) as ID
    }

    pub fn add_condition(&mut self, condition: Condition) {
        self.conditions.push(condition);
        self.proofs.push(Proof::new());
    }

    pub fn index_of(&self, condition: &Condition) -> Option<usize> {
        self.conditions.iter().position(|known| known == condition)
    }

    pub fn proof_of(&self, condition: &Condition) -> Option<&Proof> {
        self.index_of(condition).map(|index| &self.proofs[index])
    }

    // Adds a condition derived from the given fathers by one transfer step.
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
