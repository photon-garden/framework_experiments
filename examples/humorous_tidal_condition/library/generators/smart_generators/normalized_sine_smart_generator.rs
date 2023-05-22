use crate::prelude::*;
use nannou::math::ConvertAngle;

pub fn smart_sine() -> NormalizedSineSmartGenerator {
    NormalizedSineSmartGenerator::new()
}

pub struct NormalizedSineSmartGenerator {
    frequency: f32,
    phase: f32,
}

impl NormalizedSineSmartGenerator {
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

impl SmartGenerator<NormalizedF32, NormalizedF32> for NormalizedSineSmartGenerator {
    fn generate(&mut self, _rand: &Rand, progress: NormalizedF32) -> NormalizedF32 {
        progress
            .turns_to_rad()
            .times(self.frequency)
            .plus(self.phase)
            .normalized_sin()
    }
}
