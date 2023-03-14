use super::*;
use crate::prelude::*;
use std::ops::RangeInclusive;

pub fn uniform_random_f32() -> F32Generator {
    F32Generator {
        mode: F32GeneratorMode::UniformRandom { range: 0.0..=1.0 },
    }
}

pub struct F32Generator {
    mode: F32GeneratorMode,
}

impl F32Generator {
    pub fn generate(&mut self, rand: &Rand) -> f32 {
        match &self.mode {
            F32GeneratorMode::SameNumberEveryTime(number) => *number,
            F32GeneratorMode::UniformRandom { range } => rand.range_f32(range.clone()),
        }
    }

    pub fn range(self, range: RangeInclusive<f32>) -> F32Generator {
        match &self.mode {
            F32GeneratorMode::UniformRandom { .. } => F32Generator {
                mode: F32GeneratorMode::UniformRandom { range },
            },
            _ => self,
        }
    }
}

impl Into<F32Generator> for f32 {
    fn into(self) -> F32Generator {
        F32Generator {
            mode: F32GeneratorMode::SameNumberEveryTime(self),
        }
    }
}

pub enum F32GeneratorMode {
    SameNumberEveryTime(f32),
    UniformRandom { range: RangeInclusive<f32> },
}
