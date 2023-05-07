use crate::prelude::*;

pub struct WrapGenerator<Output> {
    generator: BoxedGenerator<Output>,
}

impl<Output> SignalGenerator for WrapGenerator<Output> {
    type Output = Output;

    fn generate(&mut self, rand: &Rand, _progress: NormalizedF32) -> Self::Output {
        self.generator.generate(rand)
    }
}
