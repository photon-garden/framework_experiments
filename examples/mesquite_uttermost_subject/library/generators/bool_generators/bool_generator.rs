use crate::prelude::*;

pub type BoolGenerator = Box<dyn Generator<Output = bool>>;

pub trait IntoBoolGenerator {
    fn into_bool_generator(self) -> BoolGenerator;
}

impl IntoBoolGenerator for bool {
    fn into_bool_generator(self) -> BoolGenerator {
        OneBoolGenerator { value: self }.into_box()
    }
}

impl<Gen> IntoBoolGenerator for Gen
where
    Gen: Generator<Output = bool> + 'static,
{
    fn into_bool_generator(self) -> BoolGenerator {
        self.into_box()
    }
}
