use crate::prelude::*;
use std::ops::RangeInclusive;

pub fn uniform_random_xy() -> UniformRandomPointGenerator {
    UniformRandomPointGenerator {
        x_range: 0.0..=1.0,
        y_range: 0.0..=1.0,
    }
}

pub struct UniformRandomPointGenerator {
    x_range: RangeInclusive<f32>,
    y_range: RangeInclusive<f32>,
}

impl Generator<(), Point2> for UniformRandomPointGenerator {
    fn generate(&mut self, rand: &Rand, _input: ()) -> Point2 {
        let x = rand.range_f32(&self.x_range);
        let y = rand.range_f32(&self.y_range);
        pt2(x, y)
    }
}

impl UniformRandomPointGenerator {
    pub fn x_range(mut self, x_range: RangeInclusive<f32>) -> Self {
        self.x_range = x_range;
        self
    }

    pub fn y_range(mut self, y_range: RangeInclusive<f32>) -> Self {
        self.y_range = y_range;
        self
    }
}
