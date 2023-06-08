use crate::prelude::*;

pub type Point2Generator<Input> = BoxedGenerator<Input, Point2>;

pub trait PointGeneratorExtension<Input>: Generator<Input, Point2> + 'static {
    fn jitter(self) -> JitterPointGenerator<Input>;
    fn x_jitter(self, x_amount: impl IntoGenerator<(), f32>) -> JitterPointGenerator<Input>;
    fn y_jitter(self, y_amount: impl IntoGenerator<(), f32>) -> JitterPointGenerator<Input>;
}

impl<Gen, Input> PointGeneratorExtension<Input> for Gen
where
    Gen: Generator<Input, Point2> + 'static,
{
    fn jitter(self) -> JitterPointGenerator<Input> {
        JitterPointGenerator {
            generator: Box::new(self),
            x_amount: 0.01.into_generator(),
            y_amount: 0.01.into_generator(),
        }
    }

    fn x_jitter(self, x_amount: impl IntoGenerator<(), f32>) -> JitterPointGenerator<Input> {
        JitterPointGenerator {
            generator: Box::new(self),
            x_amount: x_amount.into_generator(),
            y_amount: 0.01.into_generator(),
        }
    }

    fn y_jitter(self, y_amount: impl IntoGenerator<(), f32>) -> JitterPointGenerator<Input> {
        JitterPointGenerator {
            generator: Box::new(self),
            x_amount: 0.01.into_generator(),
            y_amount: y_amount.into_generator(),
        }
    }
}

impl Generator<(), Point2> for Point2 {
    fn generate(&mut self, _: &Rand, _: ()) -> Point2 {
        *self
    }
}
