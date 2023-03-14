use super::*;
use crate::prelude::*;
use std::ops::RangeInclusive;

mod same_every_time;
pub use same_every_time::*;

mod uniform_random;
pub use uniform_random::*;

mod grid;
pub use grid::*;

mod jitter;
pub use jitter::*;

pub trait PointGenerator: Generator<Output = Point2> + 'static {
    fn jitter(self) -> JitterPointGenerator;
    fn x_jitter(self, x_amount: impl IntoF32Generator) -> JitterPointGenerator;
    fn y_jitter(self, y_amount: impl IntoF32Generator) -> JitterPointGenerator;
}

impl<G> PointGenerator for G
where
    G: Generator<Output = Point2> + 'static,
{
    fn jitter(self) -> JitterPointGenerator {
        JitterPointGenerator {
            point_generator: Box::new(self),
            x_amount: 0.01.into_f32_generator(),
            y_amount: 0.01.into_f32_generator(),
        }
    }

    fn x_jitter(self, x_amount: impl IntoF32Generator) -> JitterPointGenerator {
        JitterPointGenerator {
            point_generator: Box::new(self),
            x_amount: x_amount.into_f32_generator(),
            y_amount: 0.01.into_f32_generator(),
        }
    }

    fn y_jitter(self, y_amount: impl IntoF32Generator) -> JitterPointGenerator {
        JitterPointGenerator {
            point_generator: Box::new(self),
            x_amount: 0.01.into_f32_generator(),
            y_amount: y_amount.into_f32_generator(),
        }
    }
}

pub trait IntoPointGenerator {
    fn into_point_generator(self) -> Box<dyn Generator<Output = Point2>>;
}

impl<G> IntoPointGenerator for G
where
    G: Generator<Output = Point2> + 'static,
{
    fn into_point_generator(self) -> Box<dyn Generator<Output = Point2>> {
        Box::new(self)
    }
}
