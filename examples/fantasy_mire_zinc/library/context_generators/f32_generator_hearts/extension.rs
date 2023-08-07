use crate::prelude::*;

pub trait F32GeneratorHeartExtension<Input, Context>:
    GeneratorHeart<Input, f32, Context> + Sized
where
    Input: 'static,
    Context: Sized + 'static,
{
    fn normalize_generator(
        self,
        input_min: f32,
        input_max: f32,
    ) -> MapOutputGeneratorHeart<Self, Input, f32, f32, Context> {
        self.map_output(move |output: f32| output.normalize(input_min, input_max))
    }

    fn denormalize_generator(
        self,
        new_start: f32,
        new_end: f32,
    ) -> MapOutputGeneratorHeart<Self, Input, f32, f32, Context> {
        self.map_output(move |output: f32| output.denormalize(new_start, new_end))
    }
}

impl<Gen, Input, Context> F32GeneratorHeartExtension<Input, Context> for Gen
where
    Input: 'static,
    Context: Sized + 'static,
    Gen: GeneratorHeart<Input, f32, Context> + Sized,
{
}
