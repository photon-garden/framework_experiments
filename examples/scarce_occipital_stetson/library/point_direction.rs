use crate::prelude::*;

#[derive(PartialEq, Debug, Clone)]
pub struct PointDirection {
    pub point: Point2,
    pub direction: Vec2,
}

impl PointDirection {
    pub fn angle_towards(&self, other: &Point2) -> f32 {
        let current_direction = self.direction;
        let current_point = self.point;

        let towards_other = current_point.vector_towards(other);

        current_direction.angle_between(towards_other).divided_by(TAU)
    }
}
