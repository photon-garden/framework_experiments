use super::*;
use crate::prelude::*;
use std::ops::RangeInclusive;

pub type SmartUsizeGenerator<Input> = BoxedSmartGenerator<Input, usize>;

pub fn smart_random_usize(min: usize, max: usize) -> SmartUniformRandomUsizeGenerator {
    SmartUniformRandomUsizeGenerator { range: min..=max }
}

impl SmartGenerator<(), usize> for usize {
    fn generate(&mut self, _rand: &Rand, _input: ()) -> usize {
        *self
    }
}

pub struct SmartUniformRandomUsizeGenerator {
    range: RangeInclusive<usize>,
}

impl SmartGenerator<(), usize> for SmartUniformRandomUsizeGenerator {
    fn generate(&mut self, rand: &Rand, _input: ()) -> usize {
        rand.range(self.range.clone())
    }
}
