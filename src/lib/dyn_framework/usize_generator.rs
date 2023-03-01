use super::*;
use crate::prelude::*;
use std::ops::RangeInclusive;

pub trait IntoBoxedUsizeGenerator {
    fn into_usize_generator(self) -> Box<dyn Generator<Output = usize>>;
}

impl<T> IntoBoxedUsizeGenerator for T
where
    T: Generator<Output = usize> + 'static,
{
    fn into_usize_generator(self) -> Box<dyn Generator<Output = usize>> {
        Box::new(self)
    }
}

pub struct OneUsizeGenerator {
    number: usize,
}

impl Generator for OneUsizeGenerator {
    type Output = usize;

    fn generate(&mut self, _rand: &Rand) -> usize {
        self.number
    }
}

impl IntoBoxedUsizeGenerator for usize {
    fn into_usize_generator(self) -> Box<dyn Generator<Output = usize>> {
        Box::new(OneUsizeGenerator { number: self })
    }
}

pub struct UniformRandomUsizeGenerator {
    range: RangeInclusive<usize>,
}

impl Generator for UniformRandomUsizeGenerator {
    type Output = usize;

    fn generate(&mut self, rand: &Rand) -> usize {
        rand.range(self.range.clone())
    }
}
