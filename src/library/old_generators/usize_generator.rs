use super::*;
use crate::prelude::*;
use std::ops::RangeInclusive;

pub type UsizeGenerator<Input> = BoxedGenerator<Input, usize>;

pub fn random_usize(min: usize, max: usize) -> UniformRandomUsizeGenerator {
    UniformRandomUsizeGenerator { range: min..=max }
}

impl Generator<(), usize> for usize {
    fn generate(&mut self, _rand: &Rand, _input: ()) -> usize {
        *self
    }
}

pub struct UniformRandomUsizeGenerator {
    range: RangeInclusive<usize>,
}

impl Generator<(), usize> for UniformRandomUsizeGenerator {
    fn generate(&mut self, rand: &Rand, _input: ()) -> usize {
        rand.range(self.range.clone())
    }
}
