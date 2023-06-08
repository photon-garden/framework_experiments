use crate::prelude::*;

impl<Input, Context> GeneratorHeart<Input, Hsl, Context> for Hsl
where
    Input: 'static,
    Context: Sized + 'static,
{
    fn generate_with_context(
        &mut self,
        _params: &GenerateWithContextParams<Input, Context>,
    ) -> Self {
        *self
    }
}

impl<Input> IntoContextGenerator<Input, Hsl> for Hsl
where
    Input: 'static,
{
    fn into_context_generator(self) -> ContextGenerator<Input, Self> {
        self.without_context().into_context_generator()
    }
}
