use super::*;
use crate::prelude::*;
use std::ops::RangeInclusive;

pub type F32Generator = BoxedGenerator<f32>;

pub fn uniform_random_f32() -> UniformRandomF32Generator {
    UniformRandomF32Generator { range: 0.0..=1.0 }
}

pub trait IntoF32Generator {
    fn into_f32_generator(self) -> F32Generator;
}

impl IntoF32Generator for f32 {
    fn into_f32_generator(self) -> F32Generator {
        let generator = OneF32Generator { number: self };
        Box::new(generator)
    }
}

pub struct OneF32Generator {
    number: f32,
}

impl Generator for OneF32Generator {
    type Output = f32;

    fn generate(&mut self, _rand: &Rand) -> f32 {
        self.number
    }
}

pub struct UniformRandomF32Generator {
    range: RangeInclusive<f32>,
}

impl Generator for UniformRandomF32Generator {
    type Output = f32;

    fn generate(&mut self, rand: &Rand) -> f32 {
        rand.range_f32(&self.range)
    }
}

impl UniformRandomF32Generator {
    pub fn range(self, range: RangeInclusive<f32>) -> UniformRandomF32Generator {
        UniformRandomF32Generator { range }
    }
}
