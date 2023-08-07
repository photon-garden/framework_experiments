// use crate::prelude::*;
// use std::marker::PhantomData;

// type Mapper<OldInput, NewInput> = dyn Fn(OldInput) -> NewInput;

// pub struct ContextMapInputGeneratorHeart<'a, Gen, OldInput, NewInput, Output, Context>
// where
//     OldInput: 'static,
//     NewInput: 'static,
//     Output: Clone + 'static,
//     Context: Sized + 'static,
//     Gen: GeneratorHeart<OldInput, Output, Context>,
// {
//     output: PhantomData<Output>,
//     context: PhantomData<Context>,
//     generator: Gen,
//     mapper: Box<Mapper<&'a NewInput, OldInput>>,
// }

// impl<'a, Gen, OldInput, NewInput, Output, Context>
//     ContextMapInputGeneratorHeart<'a, Gen, OldInput, NewInput, Output, Context>
// where
//     OldInput: 'static,
//     NewInput: 'static,
//     Output: Clone + 'static,
//     Context: Sized + 'static,
//     Gen: GeneratorHeart<OldInput, Output, Context>,
// {
//     pub fn new<Mapper>(generator: Gen, mapper: Mapper) -> Self
//     where
//         Mapper: Fn(&'a NewInput) -> OldInput + 'static,
//     {
//         Self {
//             output: PhantomData,
//             context: PhantomData,
//             generator,
//             mapper: mapper.into_box(),
//         }
//     }
// }

// impl<'a, Gen, OldInput, NewInput, Output, Context> GeneratorHeart<NewInput, Output, Context>
//     for ContextMapInputGeneratorHeart<'a, Gen, OldInput, NewInput, Output, Context>
// where
//     OldInput: 'static,
//     NewInput: 'static,
//     Output: Clone + 'static,
//     Context: Sized + 'static,
//     Gen: GeneratorHeart<OldInput, Output, Context>,
// {
//     fn generate_with_context(
//         &mut self,
//         params: &GenerateWithContextParams<NewInput, Context>,
//     ) -> Output {
//         let old_input = (self.mapper)(params.input);
//         let new_params = GenerateWithContextParams {
//             input: &old_input,
//             context: params.context,
//             rand: params.rand,
//         };

//         self.generator.generate_with_context(&new_params)
//     }
// }
