use super::generator::Generator;
use crate::prelude::*;

pub type Point2Generator = BoxedGenerator<Point2>;

pub struct JitterPointGenerator {
    pub point_generator: Point2Generator,
    pub x_amount: F32Generator,
    pub y_amount: F32Generator,
}

impl Generator for JitterPointGenerator {
    type Output = Point2;

    fn generate(&mut self, rand: &Rand) -> Point2 {
        let point = self.point_generator.generate(rand);

        let x_jitter = rand.zero_to_one() * self.x_amount.generate(rand);
        let y_jitter = rand.zero_to_one() * self.y_amount.generate(rand);

        let x = point.x + x_jitter;
        let y = point.y + y_jitter;

        pt2(x, y)
    }
}
