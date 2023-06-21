use crate::prelude::*;

use super::Point2Generator;

pub struct JitterPointGenerator<Input> {
    pub generator: Point2Generator<Input>,
    pub x_amount: BoxedGenerator<(), f32>,
    pub y_amount: BoxedGenerator<(), f32>,
}

impl<Input> Generator<Input, Point2> for JitterPointGenerator<Input> {
    fn generate(&mut self, rand: &Rand, input: Input) -> Point2 {
        let point = self.generator.generate(rand, input);

        let x_jitter = rand.zero_to_one() * self.x_amount.generate(rand, ());
        let y_jitter = rand.zero_to_one() * self.y_amount.generate(rand, ());

        let x = point.x + x_jitter;
        let y = point.y + y_jitter;

        pt2(x, y)
    }
}

impl<Input> JitterPointGenerator<Input> {
    fn x_jitter(mut self, x_amount: impl IntoGenerator<(), f32>) -> JitterPointGenerator<Input> {
        self.x_amount = x_amount.into_generator();
        self
    }

    fn y_jitter(mut self, y_amount: impl IntoGenerator<(), f32>) -> JitterPointGenerator<Input> {
        self.y_amount = y_amount.into_generator();
        self
    }
}
