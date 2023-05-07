use std::ops::RangeInclusive;

use super::*;

pub type F32SignalGenerator = Box<dyn SignalGenerator<Output = f32>>;

pub struct OneNumberSignalGenerator {
    number: f32,
}

impl SignalGenerator for OneNumberSignalGenerator {
    type Output = f32;

    fn generate(&mut self, _rand: &Rand, _progress: NormalizedF32) -> f32 {
        self.number
    }
}

pub trait IntoF32SignalGenerator {
    fn into_signal_generator(self) -> F32SignalGenerator;
}

impl IntoF32SignalGenerator for f32 {
    fn into_signal_generator(self) -> F32SignalGenerator {
        Box::new(OneNumberSignalGenerator { number: self })
    }
}

impl<T> IntoF32SignalGenerator for T
where
    T: SignalGenerator<Output = f32> + 'static,
{
    fn into_signal_generator(self) -> F32SignalGenerator {
        Box::new(self)
    }
}

pub trait F32SignalGeneratorExtension: SignalGenerator<Output = f32> + Sized {
    fn normalize(self, input_min: f32, input_max: f32) -> MapSignalGenerator<Self, f32> {
        self.map(move |output: f32, _rand: &Rand, _progress: NormalizedF32| {
            output.normalize(input_min, input_max)
        })
    }

    fn denormalize(self, denormalized_range: RangeInclusive<f32>) -> MapSignalGenerator<Self, f32> {
        let start = *denormalized_range.start();
        let end = *denormalized_range.end();

        self.map(move |output: f32, _rand: &Rand, _progress: NormalizedF32| {
            output.denormalize(start, end)
        })
    }
}

impl<SignalGen> F32SignalGeneratorExtension for SignalGen where
    SignalGen: SignalGenerator<Output = f32> + Sized
{
}
