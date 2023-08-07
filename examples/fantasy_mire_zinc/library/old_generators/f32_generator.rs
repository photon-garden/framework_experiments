use std::ops::RangeInclusive;

use crate::prelude::*;

pub fn random_f32() -> UniformRandomF32Generator {
    UniformRandomF32Generator { range: 0.0..=1.0 }
}

pub trait F32GeneratorExtension<Input>: Generator<Input, f32> + Sized {
    fn normalize_generator(
        self,
        input_min: f32,
        input_max: f32,
    ) -> MapOutputGenerator<Self, Input, f32, f32> {
        self.map_output(move |output: f32| output.normalize(input_min, input_max))
    }

    fn denormalize_generator(
        self,
        new_start: f32,
        new_end: f32,
    ) -> MapOutputGenerator<Self, Input, f32, f32> {
        self.map_output(move |output: f32| output.denormalize(new_start, new_end))
    }
}

impl<Gen, In> F32GeneratorExtension<In> for Gen where Gen: Generator<In, f32> + Sized {}

impl Generator<(), f32> for f32 {
    fn generate(&mut self, _rand: &Rand, _input: ()) -> f32 {
        *self
    }
}

pub struct UniformRandomF32Generator {
    range: RangeInclusive<f32>,
}

impl Generator<(), f32> for UniformRandomF32Generator {
    fn generate(&mut self, rand: &Rand, _input: ()) -> f32 {
        rand.range_f32(&self.range)
    }
}

impl UniformRandomF32Generator {
    pub fn range(self, range: RangeInclusive<f32>) -> UniformRandomF32Generator {
        UniformRandomF32Generator { range }
    }
}
