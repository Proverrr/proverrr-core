use crate::prove::proof;
pub mod point;
pub mod segment;
pub mod ray;
pub mod line;
pub mod object;

pub type ID = u32;
//Pan-Line: Line, Segment, Ray
pub trait PanLine{}
impl PartialEq for dyn PanLine{
    fn eq(&self, other: &dyn PanLine) -> bool{
        self == other
    }
}
pub struct Figure{
    pub points: Vec<point::Point>,
    pub lines: Vec<line::Line>,
    pub segments: Vec<segment::Segment>,
    pub rays: Vec<ray::Ray>,
    pub object: Vec<object::Object>,
    pub conditions: Vec<Box<dyn proof::Condition>>,
    pub proofs: Vec<proof::Proof>
}
impl Figure {

}