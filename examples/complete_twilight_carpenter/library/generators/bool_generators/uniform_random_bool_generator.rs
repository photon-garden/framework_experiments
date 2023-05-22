use crate::prelude::*;

pub fn flip_coin(probability_of_true: f32) -> UniformRandomBoolGenerator {
    UniformRandomBoolGenerator {
        probability_of_true,
    }
}

pub struct UniformRandomBoolGenerator {
    probability_of_true: f32,
}

impl Generator for UniformRandomBoolGenerator {
    type Output = bool;

    fn generate(&mut self, rand: &Rand) -> bool {
        rand.flip_coin(self.probability_of_true)
    }
}
