use super::*;
use crate::prelude::*;
use nannou::math::ConvertAngle;
use std::ops::RangeInclusive;

pub struct SignalGenerator {
    mode: SignalGeneratorMode,
}

pub fn sine() -> SignalGenerator {
    SignalGenerator {
        mode: SignalGeneratorMode::NormalizedSine {
            frequency: 1.0,
            amplitude_range: 0.0..=1.0,
            phase: 0.0,
        },
    }
}

impl SignalGenerator {
    pub fn frequency(self, frequency: f32) -> SignalGenerator {
        match self.mode {
            SignalGeneratorMode::NormalizedSine {
                phase,
                amplitude_range,
                ..
            } => SignalGenerator {
                mode: SignalGeneratorMode::NormalizedSine {
                    frequency,
                    amplitude_range,
                    phase,
                },
            },
            _ => self,
        }
    }

    pub fn amplitude_range(self, amplitude_range: RangeInclusive<f32>) -> SignalGenerator {
        match self.mode {
            SignalGeneratorMode::NormalizedSine {
                frequency, phase, ..
            } => SignalGenerator {
                mode: SignalGeneratorMode::NormalizedSine {
                    frequency,
                    amplitude_range,
                    phase,
                },
            },
            _ => self,
        }
    }

    pub fn generate(&mut self, rand: &Rand, progress: NormalizedF32) -> f32 {
        match &mut self.mode {
            SignalGeneratorMode::NormalizedSine {
                frequency,
                amplitude_range,
                phase,
            } => progress
                .turns_to_rad()
                .times(*frequency)
                .plus(*phase)
                .normalized_sin()
                .denormalize_to_range(amplitude_range),
            SignalGeneratorMode::SameNumberEveryTime(number) => *number,
            SignalGeneratorMode::WrapF32Generator(generator) => generator.generate(rand),
        }
    }
}

pub enum SignalGeneratorMode {
    SameNumberEveryTime(f32),
    NormalizedSine {
        frequency: f32,
        amplitude_range: RangeInclusive<f32>,
        phase: f32,
    },
    WrapF32Generator(F32Generator),
}

impl Into<SignalGenerator> for f32 {
    fn into(self) -> SignalGenerator {
        SignalGenerator {
            mode: SignalGeneratorMode::SameNumberEveryTime(self),
        }
    }
}

impl Into<SignalGenerator> for F32Generator {
    fn into(self) -> SignalGenerator {
        SignalGenerator {
            mode: SignalGeneratorMode::WrapF32Generator(self),
        }
    }
}
