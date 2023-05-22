use crate::prelude::*;

impl SmartGenerator<(), Hsl> for Hsl {
    fn generate(&mut self, _rand: &Rand, _input: ()) -> Hsl {
        *self
    }
}
