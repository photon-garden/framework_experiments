use crate::prelude::*;

pub struct CrystallizedSignalGenerator<SignalGen>
where
    SignalGen: SignalGenerator,
    SignalGen::Output: Clone,
{
    signal_generator: SignalGen,
    last_draw_index: usize,
    previous_output: Option<SignalGen::Output>,
}

impl<SignalGen> CrystallizedSignalGenerator<SignalGen>
where
    SignalGen: SignalGenerator,
    SignalGen::Output: Clone,
{
    pub fn new(signal_generator: SignalGen) -> Self {
        Self {
            signal_generator,
            last_draw_index: 0,
            previous_output: None,
        }
    }

    fn update_if_in_new_draw_call(&mut self, rand: &Rand, progress: NormalizedF32) {
        let current_draw_index = *crate::library::loop_drawer::global_draw_index
            .lock()
            .unwrap();

        let draw_index_has_incremented = current_draw_index > self.last_draw_index;
        if draw_index_has_incremented || self.previous_output.is_none() {
            self.last_draw_index = current_draw_index;

            let new_output = self.signal_generator.generate(rand, progress);
            self.previous_output = new_output.into_some();
        }
    }
}

impl<SignalGen> SignalGenerator for CrystallizedSignalGenerator<SignalGen>
where
    SignalGen: SignalGenerator,
    SignalGen::Output: Clone,
{
    type Output = SignalGen::Output;

    fn generate(&mut self, rand: &Rand, progress: NormalizedF32) -> Self::Output {
        self.update_if_in_new_draw_call(rand, progress);
        self.previous_output
            .clone()
            .expect("There's a bug in CrystallizedSignalGenerator. previous_output was None, but should have been set in update_if_in_new_draw_call.")
    }
}
