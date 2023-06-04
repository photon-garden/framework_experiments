// use crate::prelude::*;

// pub struct GeneratorSoul<Input, Output, Context, State> {
//     generate_from_context_inner: GenerateFromContext<Input, Output, Context, State>,
//     state: State,
// }

// impl<Input, Output, Context, State> GeneratorSoul<Input, Output, Context, State> {
//     fn new<Generate>(generate_from_context: Generate, state: State) -> Self
//     where
//         Generate: FnMut(&mut GenerateFromContextParams<Input, Context, State>) -> Output + 'static,
//     {
//         Self {
//             generate_from_context: Box::new(generate_from_context),
//             state,
//         }
//     }

//     fn generate_from_context(
//         &mut self,
//         params: &GenerateWithContextParams<Input, Context>,
//     ) -> Output {
//         let mut params = GenerateFromContextParams {
//             rand: params.rand,
//             input: params.input,
//             context: params.context,
//             state: &mut self.state,
//         };
//         (self.generate_from_context_inner)(&mut params)
//     }
// }

// struct GenerateFromContextParams<'a, Input, Context, State> {
//     rand: &'a Rand,
//     input: &'a Input,
//     context: &'a Context,
//     state: &'a mut State,
// }

// type GenerateFromContext<Input, Output, Context, State> =
//     Box<dyn FnMut(&mut GenerateFromContextParams<Input, Context, State>) -> Output>;
