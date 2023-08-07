use crate::prelude::*;
use std::ops::RangeInclusive;

pub fn uniform_random_usize(min: usize, max: usize) -> UniformRandomUsize {
    UniformRandomUsize { range: min..=max }
}

pub struct UniformRandomUsize {
    range: RangeInclusive<usize>,
}

impl<Context> GeneratorHeart<(), usize, Context> for UniformRandomUsize
where
    Context: Sized + 'static,
{
    fn generate_with_context(&mut self, params: &GenerateWithContextParams<(), Context>) -> usize {
        params.rand.range(self.range.clone())
    }
}

impl IntoContextGenerator<(), usize> for UniformRandomUsize {
    fn into_context_generator(self) -> ContextGenerator<(), usize> {
        self.without_context().into_context_generator()
    }
}
