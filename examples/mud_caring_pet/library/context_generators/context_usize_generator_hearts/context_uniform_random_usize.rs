use crate::prelude::*;
use std::ops::RangeInclusive;

pub fn context_uniform_random_usize(min: usize, max: usize) -> ContextUniformRandomUsize {
    ContextUniformRandomUsize { range: min..=max }
}

pub struct ContextUniformRandomUsize {
    range: RangeInclusive<usize>,
}

impl<Context> GeneratorHeart<(), usize, Context> for ContextUniformRandomUsize
where
    Context: Sized + 'static,
{
    fn generate_with_context(&mut self, params: &GenerateWithContextParams<(), Context>) -> usize {
        params.rand.range(self.range.clone())
    }
}

impl IntoContextGenerator<(), usize> for ContextUniformRandomUsize {
    fn into_context_generator(self) -> ContextGenerator<(), usize> {
        self.without_context().into_context_generator()
    }
}
