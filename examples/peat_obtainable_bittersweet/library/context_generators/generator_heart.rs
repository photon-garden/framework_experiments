use crate::prelude::*;

pub trait GeneratorHeart<Input, Output, Context>
where
    Input: 'static,
    Output: Clone + 'static,
    Context: Sized + 'static,
{
    fn generate_with_context(
        &mut self,
        params: &GenerateWithContextParams<Input, Context>,
    ) -> Output;

    fn with_context(
        self,
        context: Context,
        save_context: impl FnMut(&mut Context, Output) + 'static,
    ) -> ContextProvider<Input, Output, Context>
    where
        Self: Sized + 'static,
    {
        ContextProvider::new(self, context, save_context)
    }
}

pub trait WithoutContext<Input, Output>
where
    Input: 'static,
    Output: Clone + 'static,
{
    fn without_context(self) -> ContextProvider<Input, Output, ()>
    where
        Self: Sized + 'static;

    // fn into_context_generator(self) -> ContextGenerator<Input, Output>
    // where
    //     Self: Sized + 'static;
}

impl<Input, Output, Gen> WithoutContext<Input, Output> for Gen
where
    Input: 'static,
    Output: Clone + 'static,
    Gen: GeneratorHeart<Input, Output, ()> + 'static + Sized,
{
    fn without_context(self) -> ContextProvider<Input, Output, ()>
    where
        Self: Sized + 'static,
    {
        ContextProvider::new(self, (), |_, _| {})
    }

    // fn into_context_generator(self) -> ContextGenerator<Input, Output>
    // where
    //     Self: Sized + 'static,
    // {
    //     self.without_context().into_context_generator()
    // }
}

pub struct GenerateWithContextParams<'a, Input, Context> {
    pub rand: &'a Rand,
    pub context: &'a Context,
    pub input: &'a Input,
}

// pub trait IntoContextProvider<Input, Output, Context>
// where
//     Input: 'static,
//     Output: Clone + 'static,
//     Context: Sized + 'static,
// {
//     fn into_context_provider(self) -> ContextProvider<Input, Output, Context>;
// }

// impl<Input, Output, Gen> IntoContextProvider<Input, Output, ()> for Gen
// where
//     Input: 'static,
//     Output: Clone + 'static,
//     Gen: GeneratorHeart<Input, Output, ()> + 'static + Sized,
// {
//     fn into_context_provider(self) -> ContextProvider<Input, Output, ()> {
//         ContextProvider::new(self, (), |_, _| {})
//     }
// }

// impl<Input, Output, Context> IntoContextProvider<Input, Output, Context>
//     for ContextProvider<Input, Output, Context>
// where
//     Input: 'static,
//     Output: Clone + 'static,
//     Context: Sized + 'static,
// {
//     fn into_context_provider(self) -> ContextProvider<Input, Output, Context> {
//         self
//     }
// }
