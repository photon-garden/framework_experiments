use crate::prelude::*;

use super::SmartPoint2Generator;

pub struct SmartJitterPointGenerator<Input> {
    pub generator: SmartPoint2Generator<Input>,
    pub x_amount: BoxedSmartGenerator<(), f32>,
    pub y_amount: BoxedSmartGenerator<(), f32>,
}

impl<Input> SmartGenerator<Input, Point2> for SmartJitterPointGenerator<Input> {
    fn generate(&mut self, rand: &Rand, input: Input) -> Point2 {
        let point = self.generator.generate(rand, input);

        let x_jitter = rand.zero_to_one() * self.x_amount.generate(rand, ());
        let y_jitter = rand.zero_to_one() * self.y_amount.generate(rand, ());

        let x = point.x + x_jitter;
        let y = point.y + y_jitter;

        pt2(x, y)
    }
}

impl<Input> SmartJitterPointGenerator<Input> {
    fn x_jitter(
        mut self,
        x_amount: impl IntoSmartGenerator<(), f32>,
    ) -> SmartJitterPointGenerator<Input> {
        self.x_amount = x_amount.into_smart_generator();
        self
    }

    fn y_jitter(
        mut self,
        y_amount: impl IntoSmartGenerator<(), f32>,
    ) -> SmartJitterPointGenerator<Input> {
        self.y_amount = y_amount.into_smart_generator();
        self
    }
}
