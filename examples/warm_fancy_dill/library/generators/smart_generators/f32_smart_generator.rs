use crate::prelude::*;

pub type F32SmartGenerator<In> = BoxedSmartGenerator<In, f32>;

pub trait F32SmartGeneratorExtension<Input>: SmartGenerator<Input, f32> + Sized {
    fn normalize_generator(
        self,
        input_min: f32,
        input_max: f32,
    ) -> MapOutputSmartGenerator<Self, Input, f32, f32> {
        self.map_output(move |output: f32| output.normalize(input_min, input_max))
    }

    fn denormalize_generator(
        self,
        new_start: f32,
        new_end: f32,
    ) -> MapOutputSmartGenerator<Self, Input, f32, f32> {
        self.map_output(move |output: f32| output.denormalize(new_start, new_end))
    }
}

impl<SmartGen, In> F32SmartGeneratorExtension<In> for SmartGen where
    SmartGen: SmartGenerator<In, f32> + Sized
{
}
