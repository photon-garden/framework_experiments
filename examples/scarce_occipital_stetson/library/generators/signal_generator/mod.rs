use crate::prelude::*;

mod f32_signal_generator;
pub use f32_signal_generator::*;

mod normalized_sine_signal_generator;
pub use normalized_sine_signal_generator::*;

mod wrap_generator;
pub use wrap_generator::*;

pub type BoxedSignalGenerator<T> = Box<dyn SignalGenerator<Output = T>>;

pub trait SignalGenerator {
    type Output;

    // Could we be more generic here? Instead of a progress argument, maybe
    // it could be some kind of generic context argument?
    fn generate(&mut self, rand: &Rand, progress: NormalizedF32) -> Self::Output;
}

impl<Gen> SignalGenerator for Gen
where
    Gen: Generator,
{
    type Output = Gen::Output;

    fn generate(&mut self, rand: &Rand, _progress: NormalizedF32) -> Self::Output {
        self.generate(rand)
    }
}
