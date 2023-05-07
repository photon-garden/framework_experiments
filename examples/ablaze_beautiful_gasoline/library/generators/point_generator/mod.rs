use super::*;
use crate::prelude::*;

mod one_point_generator;
pub use one_point_generator::*;

mod uniform_random_point_generator;
pub use uniform_random_point_generator::*;

mod grid_point_generator;
pub use grid_point_generator::*;

mod jitter_point_generator;
pub use jitter_point_generator::*;

pub type Point2Generator = Box<dyn Generator<Output = Point2>>;

pub trait JitterablePointGenerator: Generator<Output = Point2> + 'static {
    fn jitter(self) -> JitterPointGenerator;
    fn x_jitter(self, x_amount: impl IntoF32Generator) -> JitterPointGenerator;
    fn y_jitter(self, y_amount: impl IntoF32Generator) -> JitterPointGenerator;
}

impl<G> JitterablePointGenerator for G
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
