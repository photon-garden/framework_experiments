use crate::prelude::*;
use nannou::math::ConvertAngle;

pub fn sine() -> NormalizedSineGenerator {
    NormalizedSineGenerator::new()
}

pub struct NormalizedSineGenerator {
    frequency: f32,
    phase: f32,
}

impl NormalizedSineGenerator {
    pub fn new() -> Self {
        Self {
            frequency: 1.0,
            phase: 0.0,
        }
    }

    pub fn frequency(mut self, frequency: f32) -> Self {
        self.frequency = frequency;
        self
    }

    pub fn phase(mut self, phase: f32) -> Self {
        self.phase = phase;
        self
    }
}

impl Generator<NormalizedF32, NormalizedF32> for NormalizedSineGenerator {
    fn generate(&mut self, _rand: &Rand, progress: NormalizedF32) -> NormalizedF32 {
        progress
            .turns_to_rad()
            .times(self.frequency)
            .plus(self.phase)
            .normalized_sin()
    }
}
