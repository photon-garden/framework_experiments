use super::*;
// use crate::prelude::*;
use crate::library::extensions::*;
use crate::prelude::NormalizedF32;
use crate::prelude::Rand;
use nannou::math::ConvertAngle;
use std::ops::RangeInclusive;

pub type BoxedSignalGenerator<T> = Box<dyn SignalGenerator<Output = T>>;

pub trait SignalGenerator {
    type Output;
    fn generate(&mut self, rand: &Rand, progress: NormalizedF32) -> Self::Output;
}

pub fn sine() -> NormalizedSineSignalGenerator {
    NormalizedSineSignalGenerator::new()
}

pub struct OneNumberSignalGenerator {
    number: f32,
}

impl SignalGenerator for OneNumberSignalGenerator {
    type Output = f32;
    fn generate(&mut self, _rand: &Rand, _progress: NormalizedF32) -> f32 {
        self.number
    }
}

impl IntoF32SignalGenerator for f32 {
    fn into_signal_generator(self) -> Box<dyn SignalGenerator<Output = f32>> {
        Box::new(OneNumberSignalGenerator { number: self })
    }
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

pub trait IntoF32SignalGenerator {
    fn into_signal_generator(self) -> Box<dyn SignalGenerator<Output = f32>>;
}

impl<T> IntoF32SignalGenerator for T
where
    T: SignalGenerator<Output = f32> + 'static,
{
    fn into_signal_generator(self) -> Box<dyn SignalGenerator<Output = f32>> {
        Box::new(self)
    }
}

pub struct WrapGenerator<Output> {
    generator: Box<dyn Generator<Output = Output>>,
}

impl<Output> SignalGenerator for WrapGenerator<Output> {
    type Output = Output;

    fn generate(&mut self, rand: &Rand, _progress: NormalizedF32) -> Self::Output {
        self.generator.generate(rand)
    }
}
