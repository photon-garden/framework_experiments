use crate::prelude::*;
use std::iter::{Enumerate, Take};

pub trait ExactSizeIteratorExtension<I, T>
where
    I: ExactSizeIterator<Item = T>,
    Self: std::marker::Sized,
{
    fn enumerate_normalized(self) -> EnumerateNormalizedIterator<Enumerate<I>, T>;
    fn take_normalized(self, proportion_to_take: NormalizedF32) -> Take<Self>;
}

impl<I, T> ExactSizeIteratorExtension<I, T> for I
where
    I: ExactSizeIterator<Item = T>,
{
    fn enumerate_normalized(self) -> EnumerateNormalizedIterator<Enumerate<I>, T> {
        let len = self.len();
        let max_index = len - 1;

        EnumerateNormalizedIterator {
            iter: self.enumerate(),
            index: 0,
            max_index,
        }
    }

    fn take_normalized(self, proportion_to_take: NormalizedF32) -> Take<Self> {
        let len = self.len() as f32;
        let num_elements_to_take_f32 = proportion_to_take * len;
        let num_elements_to_take = num_elements_to_take_f32 as usize;
        self.take(num_elements_to_take)
    }
}

pub struct EnumerateNormalizedIterator<I, T>
where
    I: Iterator<Item = (usize, T)>,
{
    iter: I,
    index: usize,
    max_index: usize,
}

impl<I, T> Iterator for EnumerateNormalizedIterator<I, T>
where
    I: Iterator<Item = (usize, T)>,
{
    type Item = (f32, T);
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(index, value)| {
            let normalized_index = if self.max_index == 0 {
                0.0
            } else {
                index as f32 / self.max_index as f32
            };

            (normalized_index, value)
        })
    }
}
