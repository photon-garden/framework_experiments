use crate::prelude::*;

impl<Context> ContextGenerator<(), Point2, Context> for Point2
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
