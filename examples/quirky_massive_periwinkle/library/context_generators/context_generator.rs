use crate::prelude::*;

pub trait ContextGenerator<Input, Output, Context>
where
    Input: 'static,
    Output: Clone + 'static,
    Context: Sized + 'static,
{
    fn generate_with_context(
        &mut self,
        params: &GenerateWithContextParams<Input, Context>,
    ) -> Output;
}

pub struct GenerateWithContextParams<'a, Input, Context> {
    pub rand: &'a Rand,
    pub context: &'a Context,
    pub input: &'a Input,
}

pub trait IntoContextProvider<Input, Output, Context>:
    ContextGenerator<Input, Output, Context> + 'static + Sized
where
    Input: 'static,
    Output: Clone + 'static,
    Context: Sized + 'static,
{
    fn into_context_provider(
        self,
        context: Context,
        output_saver: impl FnMut(&mut Context, Output) + 'static,
    ) -> ContextProvider<Input, Output, Context> {
        ContextProvider::new(self, context, output_saver)
    }
}

impl<Input, Output, Context, Gen> IntoContextProvider<Input, Output, Context> for Gen
where
    Input: 'static,
    Output: Clone + 'static,
    Context: Sized + 'static,
    Gen: ContextGenerator<Input, Output, Context> + 'static + Sized,
{
}
