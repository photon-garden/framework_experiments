use crate::prelude::*;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct UsizePoint2 {
    pub x: usize,
    pub y: usize,
}

pub fn usize_pt2(x: usize, y: usize) -> UsizePoint2 {
    UsizePoint2 { x, y }
}
