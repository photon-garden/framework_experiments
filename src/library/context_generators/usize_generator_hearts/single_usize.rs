use crate::prelude::*;

impl<Context> GeneratorHeart<(), usize, Context> for usize
where
    Context: Sized + 'static,
{
    fn generate_with_context(&mut self, _params: &GenerateWithContextParams<(), Context>) -> Self {
        *self
    }
}

impl IntoContextGenerator<(), usize> for usize {
    fn into_context_generator(self) -> ContextGenerator<(), Self> {
        self.without_context().into_context_generator()
    }
}
