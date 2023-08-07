use crate::prelude::*;
use std::marker::PhantomData;

// A crystallized generator works differently from a normal generator. A normal generator
// typically generates a new value every time it's called. But a crystallized generator only
// generates a new value once per draw call.

pub struct CrystallizedGeneratorHeart<Gen, Input, Output, Context>
where
    Input: 'static,
    Output: Clone + 'static,
    Context: Sized + 'static,
    Gen: GeneratorHeart<Input, Output, Context>,
{
    generator: Gen,
    last_draw_index: usize,
    previous_output: Option<Output>,
    input: PhantomData<Input>,
    context: PhantomData<Context>,
}

impl<Gen, Input, Output, Context> CrystallizedGeneratorHeart<Gen, Input, Output, Context>
where
    Input: 'static,
    Output: Clone + 'static,
    Context: Sized + 'static,
    Gen: GeneratorHeart<Input, Output, Context>,
{
    pub fn new(generator: Gen) -> Self {
        Self {
            generator,
            last_draw_index: 0,
            previous_output: None,
            input: PhantomData,
            context: PhantomData,
        }
    }

    fn output_that_updates_if_in_new_draw_call(
        &mut self,
        params: &GenerateWithContextParams<Input, Context>,
    ) -> Output {
        let current_draw_index = *crate::library::loop_drawer::global_draw_index
            .lock()
            .unwrap();

        let draw_index_has_incremented = current_draw_index > self.last_draw_index;
        let should_update = draw_index_has_incremented || self.previous_output.is_none();

        if should_update {
            self.last_draw_index = current_draw_index;

            let new_output = self.generator.generate_with_context(params);
            self.previous_output = new_output.into_some();
        }

        self.previous_output.clone().expect("There's a bug in CrystallizedOutputGenerator. self.previous_output should've been Some but was None.")
    }
}

impl<Gen, Input, Output, Context> GeneratorHeart<Input, Output, Context>
    for CrystallizedGeneratorHeart<Gen, Input, Output, Context>
where
    Input: 'static,
    Output: Clone + 'static,
    Context: Sized + 'static,
    Gen: GeneratorHeart<Input, Output, Context>,
{
    fn generate_with_context(
        &mut self,
        params: &GenerateWithContextParams<Input, Context>,
    ) -> Output {
        self.output_that_updates_if_in_new_draw_call(params)
    }
}

impl<Gen, Input, Output> IntoContextGenerator<Input, Output>
    for CrystallizedGeneratorHeart<Gen, Input, Output, ()>
where
    Input: 'static,
    Output: Clone + 'static,
    Gen: GeneratorHeart<Input, Output, ()> + 'static,
{
    fn into_context_generator(self) -> ContextGenerator<Input, Output> {
        self.without_context().into_context_generator()
    }
}
