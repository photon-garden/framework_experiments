use crate::prelude::*;
use std::marker::PhantomData;

type UpdateContext<Output, Context> = Box<dyn FnMut(&mut Context, Output)>;

pub trait ContextProviderInterface<Input, Output>
where
    Input: 'static,
    Output: Clone + 'static,
{
    fn generate(&mut self, rand: &Rand, input: Input) -> Output;
}

impl<Input, Output, Context> ContextProviderInterface<Input, Output>
    for ContextProvider<Input, Output, Context>
where
    Input: 'static,
    Output: Clone + 'static,
    Context: Sized + 'static,
{
    fn generate(&mut self, rand: &Rand, input: Input) -> Output {
        self.context_provider_generate(rand, input)
    }
}

pub struct ContextProvider<Input, Output, Context>
where
    Input: 'static,
    Output: Clone + 'static,
    Context: Sized + 'static,
{
    pub heart: Box<dyn GeneratorHeart<Input, Output, Context>>,
    pub context: Context,
    update_context: UpdateContext<Output, Context>,
    input: PhantomData<Input>,
    output: PhantomData<Output>,
}

impl<Input, Output, Context> ContextProvider<Input, Output, Context>
where
    Input: 'static,
    Output: Clone + 'static,
    Context: Sized + 'static,
{
    pub fn new<Gen>(
        heart: Gen,
        context: Context,
        update_context: impl FnMut(&mut Context, Output) + 'static,
    ) -> Self
    where
        Gen: GeneratorHeart<Input, Output, Context> + Sized + 'static,
    {
        Self {
            heart: heart.into_box(),
            context,
            update_context: update_context.into_box(),
            input: PhantomData,
            output: PhantomData,
        }
    }

    fn context_provider_generate(&mut self, rand: &Rand, input: Input) -> Output {
        let params = GenerateWithContextParams {
            rand,
            context: &self.context,
            input: &input,
        };
        let output = self.heart.generate_with_context(&params);

        let context = &mut self.context;
        (self.update_context)(context, output.clone());

        output
    }

    pub fn filter(
        self,
        filter: impl Fn(ContextFilterParams<Input, Output, Context>) -> bool + 'static,
    ) -> ContextProvider<Input, Output, Context> {
        ContextProvider {
            heart: ContextFilterGenerator::new(self.heart, filter).into_box(),
            context: self.context,
            update_context: self.update_context,
            input: PhantomData,
            output: PhantomData,
        }
    }
}
