use crate::prelude::*;

impl<Context> GeneratorHeart<(), f32, Context> for f32
where
    Context: Sized + 'static,
{
    fn generate_with_context(&mut self, _params: &GenerateWithContextParams<(), Context>) -> Self {
        *self
    }
}

impl IntoContextGenerator<(), f32> for f32 {
    fn into_context_generator(self) -> ContextGenerator<(), Self> {
        self.without_context().into_context_generator()
    }
}
