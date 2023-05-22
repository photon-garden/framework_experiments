use crate::prelude::*;
use std::marker::PhantomData;

// A crystallized generator works differently from a normal generator. A normal generator
// typically generates a new value every time it's called. But a crystallized generator only
// generates a new value once per draw call.

pub struct CrystallizedGenerator<Gen, Input, Output>
where
    Gen: Generator<Input, Output>,
    Output: Clone,
{
    generator: Gen,
    last_draw_index: usize,
    previous_output: Option<Output>,
    input: PhantomData<Input>,
}

impl<Gen, Input, Output> CrystallizedGenerator<Gen, Input, Output>
where
    Gen: Generator<Input, Output>,
    Output: Clone,
{
    pub fn new(generator: Gen) -> Self {
        Self {
            generator,
            last_draw_index: 0,
            previous_output: None,
            input: PhantomData,
        }
    }

    fn update_if_in_new_draw_call(&mut self, rand: &Rand, input: Input) {
        let current_draw_index = *crate::library::loop_drawer::global_draw_index
            .lock()
            .unwrap();

        let draw_index_has_incremented = current_draw_index > self.last_draw_index;
        if draw_index_has_incremented || self.previous_output.is_none() {
            self.last_draw_index = current_draw_index;

            let new_output = self.generator.generate(rand, input);
            self.previous_output = new_output.into_some();
        }
    }
}

impl<Gen, Input, Output> Generator<Input, Output> for CrystallizedGenerator<Gen, Input, Output>
where
    Gen: Generator<Input, Output>,
    Output: Clone,
{
    fn generate(&mut self, rand: &Rand, input: Input) -> Output {
        self.update_if_in_new_draw_call(rand, input);
        self.previous_output
            .clone()
            .expect("There's a bug in CrystallizedGenerator. previous_output was None, but should have been set in update_if_in_new_draw_call.")
    }
}
