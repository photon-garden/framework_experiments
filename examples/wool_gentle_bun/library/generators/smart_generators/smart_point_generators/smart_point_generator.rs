use crate::prelude::*;

pub type SmartPoint2Generator<Input> = BoxedSmartGenerator<Input, Point2>;

pub trait SmartPointGeneratorExtension<Input>: SmartGenerator<Input, Point2> + 'static {
    fn jitter(self) -> SmartJitterPointGenerator<Input>;
    fn x_jitter(self, x_amount: impl IntoSmartF32Generator<()>)
    -> SmartJitterPointGenerator<Input>;
    fn y_jitter(self, y_amount: impl IntoSmartF32Generator<()>)
    -> SmartJitterPointGenerator<Input>;
}

impl<SmartGen, Input> SmartPointGeneratorExtension<Input> for SmartGen
where
    SmartGen: SmartGenerator<Input, Point2> + 'static,
{
    fn jitter(self) -> SmartJitterPointGenerator<Input> {
        SmartJitterPointGenerator {
            generator: Box::new(self),
            x_amount: 0.01.into_smart_f32_generator(),
            y_amount: 0.01.into_smart_f32_generator(),
        }
    }

    fn x_jitter(
        self,
        x_amount: impl IntoSmartF32Generator<()>,
    ) -> SmartJitterPointGenerator<Input> {
        SmartJitterPointGenerator {
            generator: Box::new(self),
            x_amount: x_amount.into_smart_f32_generator(),
            y_amount: 0.01.into_smart_f32_generator(),
        }
    }

    fn y_jitter(
        self,
        y_amount: impl IntoSmartF32Generator<()>,
    ) -> SmartJitterPointGenerator<Input> {
        SmartJitterPointGenerator {
            generator: Box::new(self),
            x_amount: 0.01.into_smart_f32_generator(),
            y_amount: y_amount.into_smart_f32_generator(),
        }
    }
}

impl SmartGenerator<(), Point2> for Point2 {
    fn generate(&mut self, _: &Rand, _: ()) -> Point2 {
        *self
    }
}
