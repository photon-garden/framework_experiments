use crate::prelude::*;

impl<Context> GeneratorHeart<(), Point2, Context> for Point2
where
    Context: Sized + 'static,
{
    fn generate_with_context(
        &mut self,
        _params: &GenerateWithContextParams<(), Context>,
    ) -> Point2 {
        *self
    }
}

impl IntoContextGenerator<(), Point2> for Point2 {
    fn into_context_generator(self) -> ContextGenerator<(), Self> {
        self.without_context().into_context_generator()
    }
}

impl Default for ContextGenerator<(), Point2> {
    fn default() -> Self {
        pt2(0.5, 0.5).into_context_generator()
    }
}
