use crate::prelude::*;

pub trait PointGeneratorExtension: Generator<Output = Point2> + 'static {
    fn jitter(self) -> JitterPointGenerator;
    fn x_jitter(self, x_amount: impl IntoF32Generator) -> JitterPointGenerator;
    fn y_jitter(self, y_amount: impl IntoF32Generator) -> JitterPointGenerator;
}

impl<G> PointGeneratorExtension for G
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

pub type Point2Generator = Box<dyn Generator<Output = Point2>>;

pub trait IntoPointGenerator {
    fn into_point_generator(self) -> Point2Generator;
}

impl<G> IntoPointGenerator for G
where
    G: Generator<Output = Point2> + 'static,
{
    fn into_point_generator(self) -> Point2Generator {
        Box::new(self)
    }
}
