use super::generator::Generator;
use crate::prelude::*;

pub struct JitterPointGenerator {
    pub point_generator: Box<dyn Generator<Output = Point2>>,
    pub x_amount: Box<dyn Generator<Output = f32>>,
    pub y_amount: Box<dyn Generator<Output = f32>>,
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
