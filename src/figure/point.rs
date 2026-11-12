use crate::ratio::Ratio;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Point {
    pub x: Ratio,
    pub y: Ratio,
}

impl Point {
    pub fn new(x: Ratio, y: Ratio) -> Self {
        Point { x, y }
    }

    pub fn origin() -> Self {
        Point::new(Ratio::from(0), Ratio::from(0))
    }
}
