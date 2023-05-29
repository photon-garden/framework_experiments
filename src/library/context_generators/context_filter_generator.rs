use crate::prelude::*;
use std::marker::PhantomData;

type Filter<Input, Output, Context> =
    Box<dyn Fn(ContextFilterParams<Input, Output, Context>) -> bool>;

pub struct ContextFilterGenerator<Input, Output, Context>
where
    Input: 'static,
    Output: Clone + 'static,
    Context: Sized + 'static,
{
    generator: Box<dyn GeneratorHeart<Input, Output, Context>>,
    filter: Filter<Input, Output, Context>,
    input: PhantomData<Input>,
    output: PhantomData<Output>,
    context: PhantomData<Context>,
}

impl<Input, Output, Context> ContextFilterGenerator<Input, Output, Context>
where
    Input: 'static,
    Output: Clone + 'static,
    Context: Sized + 'static,
{
    pub fn new(
        generator: Box<dyn GeneratorHeart<Input, Output, Context>>,
        filter: impl Fn(ContextFilterParams<Input, Output, Context>) -> bool + 'static,
    ) -> Self {
        Self {
            generator,
            filter: filter.into_box(),
            input: PhantomData,
            output: PhantomData,
            context: PhantomData,
        }
    }
}

impl<Input, Output, Context> GeneratorHeart<Input, Output, Context>
    for ContextFilterGenerator<Input, Output, Context>
where
    Input: 'static,
    Output: Clone + 'static,
    Context: Sized + 'static,
{
    fn generate_with_context(
        &mut self,
        params: &GenerateWithContextParams<Input, Context>,
    ) -> Output {
        let num_tries = 1_000;

        for _ in 0..num_tries {
            let output = self.generator.generate_with_context(params);
            let filter_params = ContextFilterParams {
                generator: &self.generator,
                input: params.input,
                output: &output,
                rand: params.rand,
                context: params.context,
            };
            let should_keep = (self.filter)(filter_params);
            if should_keep {
                return output;
            }
        }

        panic!(
            "ContextFilterGenerator.generator.generate() failed to generate a value that passes the filter after {num_tries} iterations."
        );
    }
}

pub struct ContextFilterParams<'a, Input, Output, Context>
where
    Output: Clone,
{
    pub generator: &'a Box<dyn GeneratorHeart<Input, Output, Context>>,
    pub input: &'a Input,
    pub output: &'a Output,
    pub rand: &'a Rand,
    pub context: &'a Context,
}
