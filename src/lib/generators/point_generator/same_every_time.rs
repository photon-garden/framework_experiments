use super::generator::Generator;
use super::IntoPointGenerator;
use crate::prelude::*;

pub struct SameEveryTimePointGenerator {
    point: Point2,
}

impl Generator for SameEveryTimePointGenerator {
    type Output = Point2;

    fn generate(&mut self, _rand: &Rand) -> Point2 {
        self.point
    }
}

impl IntoPointGenerator for Point2 {
    fn into_point_generator(self) -> Box<dyn Generator<Output = Point2>> {
        Box::new(SameEveryTimePointGenerator { point: self })
    }
}
