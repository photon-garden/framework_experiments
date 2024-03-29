use crate::prelude::*;
use std::ops::RangeInclusive;

pub fn uniform_random_xy() -> UniformRandomPoint {
    UniformRandomPoint {
        x_range: 0.0..=1.0,
        y_range: 0.0..=1.0,
    }
}

pub struct UniformRandomPoint {
    x_range: RangeInclusive<f32>,
    y_range: RangeInclusive<f32>,
}

impl<Context> GeneratorHeart<(), Point2, Context> for UniformRandomPoint
where
    Context: Sized + 'static,
{
    fn generate_with_context(&mut self, params: &GenerateWithContextParams<(), Context>) -> Point2 {
        let x = params.rand.range_f32(&self.x_range);
        let y = params.rand.range_f32(&self.y_range);
        pt2(x, y)
    }
}

impl UniformRandomPoint {
    pub fn x_range(mut self, x_range: RangeInclusive<f32>) -> Self {
        self.x_range = x_range;
        self
    }

    pub fn y_range(mut self, y_range: RangeInclusive<f32>) -> Self {
        self.y_range = y_range;
        self
    }
}

impl IntoContextGenerator<(), Point2> for UniformRandomPoint {
    fn into_context_generator(self) -> ContextGenerator<(), Point2> {
        self.without_context().into_context_generator()
    }
}
