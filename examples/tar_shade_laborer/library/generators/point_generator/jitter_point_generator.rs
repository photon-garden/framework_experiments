use crate::prelude::*;

pub struct JitterPointGenerator {
    pub point_generator: Point2Generator,
    pub x_amount: F32Generator,
    pub y_amount: F32Generator,
}

impl Generator for JitterPointGenerator {
    type Output = Point2;

    fn generate(&mut self, rand: &Rand) -> Point2 {
        let point = self.point_generator.generate(rand);

        let x_jitter = rand.zero_to_one() * self.x_amount.generate(rand);
        let y_jitter = rand.zero_to_one() * self.y_amount.generate(rand);

        let x = point.x + x_jitter;
        let y = point.y + y_jitter;

        pt2(x, y)
    }
}

pub trait MakePointGeneratorsJitterable: Generator<Output = Point2> + 'static {
    fn jitter(self) -> JitterPointGenerator;
    fn x_jitter(self, x_amount: impl IntoF32Generator) -> JitterPointGenerator;
    fn y_jitter(self, y_amount: impl IntoF32Generator) -> JitterPointGenerator;
}

impl<G> MakePointGeneratorsJitterable for G
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
