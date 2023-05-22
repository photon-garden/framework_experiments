use crate::prelude::*;

impl Generator<(), bool> for bool {
    fn generate(&mut self, _rand: &Rand, _input: ()) -> bool {
        *self
    }
}

pub fn flip_coin(probability_of_true: f32) -> UniformRandomBoolGenerator {
    UniformRandomBoolGenerator {
        probability_of_true,
    }
}

pub struct UniformRandomBoolGenerator {
    probability_of_true: f32,
}

impl Generator<(), bool> for UniformRandomBoolGenerator {
    fn generate(&mut self, rand: &Rand, _input: ()) -> bool {
        rand.flip_coin(self.probability_of_true)
    }
}
