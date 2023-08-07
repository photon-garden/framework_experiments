use crate::prelude::*;

pub fn uniform_random_f32() -> UniformRandomF32 {
    UniformRandomF32 {}
}

pub struct UniformRandomF32 {}

impl<Context> GeneratorHeart<(), NormalizedF32, Context> for UniformRandomF32
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

impl IntoContextGenerator<(), NormalizedF32> for UniformRandomF32 {
    fn into_context_generator(self) -> ContextGenerator<(), NormalizedF32> {
        self.without_context().into_context_generator()
    }
}
