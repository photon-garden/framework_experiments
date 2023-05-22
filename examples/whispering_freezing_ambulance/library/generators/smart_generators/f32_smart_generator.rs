use std::ops::RangeInclusive;

use crate::prelude::*;

pub fn smart_random_f32() -> SmartUniformRandomF32Generator {
    SmartUniformRandomF32Generator { range: 0.0..=1.0 }
}

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

impl SmartGenerator<(), f32> for f32 {
    fn generate(&mut self, _rand: &Rand, _input: ()) -> f32 {
        *self
    }
}

pub struct SmartUniformRandomF32Generator {
    range: RangeInclusive<f32>,
}

impl SmartGenerator<(), f32> for SmartUniformRandomF32Generator {
    fn generate(&mut self, rand: &Rand, _input: ()) -> f32 {
        rand.range_f32(&self.range)
    }
}

impl SmartUniformRandomF32Generator {
    pub fn range(self, range: RangeInclusive<f32>) -> SmartUniformRandomF32Generator {
        SmartUniformRandomF32Generator { range }
    }
}
