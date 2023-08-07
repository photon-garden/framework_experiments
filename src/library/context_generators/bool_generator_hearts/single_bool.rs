use crate::prelude::*;

impl<Context> GeneratorHeart<(), bool, Context> for bool
where
    Context: Sized + 'static,
{
    fn generate_with_context(&mut self, _params: &GenerateWithContextParams<(), Context>) -> Self {
        *self
    }
}

impl IntoContextGenerator<(), bool> for bool {
    fn into_context_generator(self) -> ContextGenerator<(), Self> {
        self.without_context().into_context_generator()
    }
}
