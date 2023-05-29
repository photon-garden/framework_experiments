use crate::prelude::*;
use std::marker::PhantomData;

type OutputSaver<Output, Context> = Box<dyn FnMut(&mut Context, Output)>;

pub struct ContextProvider<Input, Output, Context>
where
    Input: 'static,
    Output: Clone + 'static,
    Context: Sized + 'static,
{
    pub generator: Box<dyn ContextGenerator<Input, Output, Context>>,
    pub context: Context,
    output_saver: OutputSaver<Output, Context>,
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
        generator: Gen,
        context: Context,
        output_saver: impl FnMut(&mut Context, Output) + 'static,
    ) -> Self
    where
        Gen: ContextGenerator<Input, Output, Context> + Sized + 'static,
    {
        Self {
            generator: generator.into_box(),
            context,
            output_saver: output_saver.into_box(),
            input: PhantomData,
            output: PhantomData,
        }
    }

    pub fn generate(&mut self, rand: &Rand, input: Input) -> Output {
        let params = GenerateWithContextParams {
            rand,
            context: &self.context,
            input: &input,
        };
        let output = self.generator.generate_with_context(&params);

        let context = &mut self.context;
        (self.output_saver)(context, output.clone());

        output
    }

    pub fn filter(
        self,
        filter: impl Fn(ContextFilterParams<Input, Output, Context>) -> bool + 'static,
    ) -> ContextProvider<Input, Output, Context> {
        ContextProvider {
            generator: ContextFilterGenerator::new(self.generator, filter).into_box(),
            context: self.context,
            output_saver: self.output_saver,
            input: PhantomData,
            output: PhantomData,
        }
    }
}

// impl<Input, Output, Context> ContextGenerator<Input, Output, Context>
//     for ContextProvider<Input, Output, Context>
// where
//     Input: 'static,
//     Output: Clone + 'static,
//     Context: Sized + 'static,
// {
//     fn generate_with_context(
//         &mut self,
//         params: &GenerateWithContextParams<Input, Context>,
//     ) -> Output {
//         self.generator.generate_with_context(params)
//     }
// }
