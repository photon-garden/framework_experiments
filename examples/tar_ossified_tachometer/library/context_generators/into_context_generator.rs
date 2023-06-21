use crate::prelude::*;

pub trait IntoContextGenerator<Input, Output>
where
    Input: 'static,
    Output: Clone + 'static,
{
    fn into_context_generator(self) -> ContextGenerator<Input, Output>;
}

impl<Input, Output, Context> IntoContextGenerator<Input, Output>
    for ContextProvider<Input, Output, Context>
where
    Input: 'static,
    Output: Clone + 'static,
    Context: Sized + 'static,
{
    fn into_context_generator(self) -> ContextGenerator<Input, Output> {
        ContextGenerator {
            context_provider: Box::new(self),
        }
    }
}

// impl<Input, Output, Gen> IntoContextGenerator<Input, Output> for Gen
// where
//     Gen: GeneratorHeart<Input, Output, ()>,
//     Input: 'static,
//     Output: Clone + 'static,
// {
//     fn into_context_generator(self) -> ContextGenerator<Input, Output> {
//         ContextGenerator {
//             context_provider: Box::new(self),
//         }
//     }
// }
