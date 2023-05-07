use crate::prelude::*;

pub struct OnePointGenerator {
    point: Point2,
}

impl Generator for OnePointGenerator {
    type Output = Point2;

    fn generate(&mut self, _rand: &Rand) -> Point2 {
        self.point
    }
}

impl IntoPointGenerator for Point2 {
    fn into_point_generator(self) -> Point2Generator {
        Box::new(OnePointGenerator { point: self })
    }
}
