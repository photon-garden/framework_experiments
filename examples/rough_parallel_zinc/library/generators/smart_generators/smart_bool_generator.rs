use crate::prelude::*;

impl SmartGenerator<(), bool> for bool {
    fn generate(&mut self, _rand: &Rand, _input: ()) -> bool {
        *self
    }
}

pub fn smart_flip_coin(probability_of_true: f32) -> SmartUniformRandomBoolGenerator {
    SmartUniformRandomBoolGenerator {
        probability_of_true,
    }
}

pub struct SmartUniformRandomBoolGenerator {
    probability_of_true: f32,
}

impl SmartGenerator<(), bool> for SmartUniformRandomBoolGenerator {
    fn generate(&mut self, rand: &Rand, _input: ()) -> bool {
        rand.flip_coin(self.probability_of_true)
    }
}
