use super::*;
use crate::prelude::NormalizedF32;
use crate::prelude::Rand;
use nannou::math::ConvertAngle;
use std::ops::RangeInclusive;

pub fn sine() -> NormalizedSineSignalGenerator {
    NormalizedSineSignalGenerator::new()
}

pub struct NormalizedSineSignalGenerator {
    frequency: f32,
    amplitude_range: RangeInclusive<f32>,
    phase: f32,
}

impl NormalizedSineSignalGenerator {
    pub fn new() -> Self {
        Self {
            frequency: 1.0,
            amplitude_range: 0.0..=1.0,
            phase: 0.0,
        }
    }

    pub fn frequency(mut self, frequency: f32) -> Self {
        self.frequency = frequency;
        self
    }

    pub fn amplitude_range(mut self, amplitude_range: RangeInclusive<f32>) -> Self {
        self.amplitude_range = amplitude_range;
        self
    }

    pub fn phase(mut self, phase: f32) -> Self {
        self.phase = phase;
        self
    }
}

impl SignalGenerator for NormalizedSineSignalGenerator {
    type Output = f32;

    fn generate(&mut self, _rand: &Rand, progress: NormalizedF32) -> Self::Output {
        progress
            .turns_to_rad()
            .times(self.frequency)
            .plus(self.phase)
            .normalized_sin()
            .denormalize_to_range(&self.amplitude_range)
    }
}
