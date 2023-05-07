use crate::prelude::*;

pub type BoolGenerator = Box<dyn Generator<Output = bool>>;

pub trait IntoBoolGenerator {
    fn into_bool_generator(self) -> BoolGenerator;
}

pub struct OneBoolGenerator {
    value: bool,
}

impl Generator for OneBoolGenerator {
    type Output = bool;

    fn generate(&mut self, _rand: &Rand) -> bool {
        self.value
    }
}

impl IntoBoolGenerator for bool {
    fn into_bool_generator(self) -> BoolGenerator {
        OneBoolGenerator { value: self }.into_box()
    }
}
