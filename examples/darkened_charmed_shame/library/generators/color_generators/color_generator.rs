use crate::prelude::*;

impl Generator<(), Hsl> for Hsl {
    fn generate(&mut self, _rand: &Rand, _input: ()) -> Hsl {
        *self
    }
}
