use crate::prelude::*;

pub fn context_uniform_random_f32() -> ContextUniformRandomF32 {
    ContextUniformRandomF32 {}
}

pub struct ContextUniformRandomF32 {}

impl<Context> GeneratorHeart<(), NormalizedF32, Context> for ContextUniformRandomF32
where
    Context: Sized + 'static,
{
    fn generate_with_context(
        &mut self,
        params: &GenerateWithContextParams<(), Context>,
    ) -> NormalizedF32 {
        params.rand.zero_to_one()
    }
}

impl IntoContextGenerator<(), NormalizedF32> for ContextUniformRandomF32 {
    fn into_context_generator(self) -> ContextGenerator<(), NormalizedF32> {
        self.without_context().into_context_generator()
    }
}
