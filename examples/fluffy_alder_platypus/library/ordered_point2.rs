use crate::prelude::*;
use ordered_float::NotNan;

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct OrderedPoint2 {
    x: ordered_float::NotNan<f32>,
    y: ordered_float::NotNan<f32>,
}

impl From<Point2> for OrderedPoint2 {
    fn from(point: Point2) -> Self {
        OrderedPoint2 {
            x: NotNan::new(point.x).unwrap(),
            y: NotNan::new(point.y).unwrap(),
        }
    }
}

impl From<&Point2> for OrderedPoint2 {
    fn from(point: &Point2) -> Self {
        OrderedPoint2 {
            x: NotNan::new(point.x).unwrap(),
            y: NotNan::new(point.y).unwrap(),
        }
    }
}
