use crate::prelude::*;

pub struct ContextGenerator<Input, Output>
where
    Input: 'static,
    Output: Clone + 'static,
{
    pub context_provider: Box<dyn ContextProviderInterface<Input, Output>>,
}

impl<Input, Output> ContextGenerator<Input, Output>
where
    Input: 'static,
    Output: Clone + 'static,
{
    pub fn generate(&mut self, rand: &Rand, input: Input) -> Output {
        self.context_provider.generate(rand, input)
    }
}
