use crate::point::{Point};

#[derive(Debug, Copy, Clone)]
pub struct ElementQuad {
    pub top_left: Point,
    pub top_right: Point,
    pub bottom_left: Point,
    pub bottom_right: Point,
}

