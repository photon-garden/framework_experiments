use super::*;
use crate::prelude::*;
use std::{ops::RangeInclusive, process::Output};

pub fn uniform_random_f32() -> DynUniformRandom {
    DynUniformRandom { range: 0.0..=1.0 }
}

pub trait IntoF32Generator {
    fn into_f32_generator(self) -> Box<dyn Generator<Output = f32>>;
}

impl IntoF32Generator for f32 {
    fn into_f32_generator(self) -> Box<dyn Generator<Output = f32>> {
        let generator = DynOneNumber { number: self };
        Box::new(generator)
    }
}

pub struct DynOneNumber {
    number: f32,
}

impl Generator for DynOneNumber {
    type Output = f32;

    fn generate(&mut self, _rand: &Rand) -> f32 {
        self.number
    }
}

pub struct DynUniformRandom {
    range: RangeInclusive<f32>,
}

impl Generator for DynUniformRandom {
    type Output = f32;

    fn generate(&mut self, rand: &Rand) -> f32 {
        rand.range_f32(&self.range)
    }
}

impl DynUniformRandom {
    pub fn range(self, range: RangeInclusive<f32>) -> DynUniformRandom {
        DynUniformRandom { range }
    }
}
