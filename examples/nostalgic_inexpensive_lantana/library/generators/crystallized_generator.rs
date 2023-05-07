use crate::prelude::*;

pub struct CrystallizedGenerator<Gen>
where
    Gen: Generator,
    Gen::Output: Clone,
{
    generator: Gen,
    last_draw_index: usize,
    previous_output: Option<Gen::Output>,
}

impl<Gen> CrystallizedGenerator<Gen>
where
    Gen: Generator,
    Gen::Output: Clone,
{
    pub fn new(generator: Gen) -> Self {
        Self {
            generator,
            last_draw_index: 0,
            previous_output: None,
        }
    }

    fn update_if_in_new_draw_call(&mut self, rand: &Rand) {
        let current_draw_index = *crate::library::loop_drawer::global_draw_index
            .lock()
            .unwrap();

        let draw_index_has_incremented = current_draw_index > self.last_draw_index;
        if draw_index_has_incremented || self.previous_output.is_none() {
            self.last_draw_index = current_draw_index;

            let new_output = self.generator.generate(rand);
            self.previous_output = new_output.into_some();
        }
    }
}

impl<Gen> Generator for CrystallizedGenerator<Gen>
where
    Gen: Generator,
    Gen::Output: Clone,
{
    type Output = Gen::Output;

    fn generate(&mut self, rand: &Rand) -> Self::Output {
        self.update_if_in_new_draw_call(rand);
        self.previous_output
            .clone()
            .expect("There's a bug in CrystallizedGenerator. previous_output was None, but should have been set in update_if_in_new_draw_call.")
    }
}
