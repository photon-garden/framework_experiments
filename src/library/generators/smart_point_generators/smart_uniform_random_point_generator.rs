use crate::prelude::*;
use std::ops::RangeInclusive;

pub fn smart_uniform_random_xy() -> SmartUniformRandomPointGenerator {
    SmartUniformRandomPointGenerator {
        x_range: 0.0..=1.0,
        y_range: 0.0..=1.0,
    }
}

pub struct SmartUniformRandomPointGenerator {
    x_range: RangeInclusive<f32>,
    y_range: RangeInclusive<f32>,
}

impl SmartGenerator<(), Point2> for SmartUniformRandomPointGenerator {
    fn generate(&mut self, rand: &Rand, _input: ()) -> Point2 {
        let x = rand.range_f32(&self.x_range);
        let y = rand.range_f32(&self.y_range);
        pt2(x, y)
    }
}

impl SmartUniformRandomPointGenerator {
    pub fn x_range(mut self, x_range: RangeInclusive<f32>) -> Self {
        self.x_range = x_range;
        self
    }

    pub fn y_range(mut self, y_range: RangeInclusive<f32>) -> Self {
        self.y_range = y_range;
        self
    }
}
