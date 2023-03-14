use super::generator::Generator;
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

impl Generator for UniformRandomPointGenerator {
    type Output = Point2;

    fn generate(&mut self, rand: &Rand) -> Point2 {
        let x = rand.range_f32(&self.x_range);
        let y = rand.range_f32(&self.y_range);
        pt2(x, y)
    }
}
