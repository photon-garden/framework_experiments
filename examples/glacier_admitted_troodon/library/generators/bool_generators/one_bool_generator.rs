use crate::prelude::*;

pub struct OneBoolGenerator {
    pub value: bool,
}

impl Generator for OneBoolGenerator {
    type Output = bool;

    fn generate(&mut self, _rand: &Rand) -> bool {
        self.value
    }
}
