use crate::prelude::*;
use std::ops::Add;
use std::ops::RangeInclusive;

pub trait VecWithCloneableElementsExtension<T> {
    fn repeat_n(self, n: usize) -> itertools::RepeatN<Vec<T>>;
    fn neighbor_indexes_around(&self, center_index: usize, radius: usize) -> RangeInclusive<usize>;
    fn repeat_first_element_at_end(self) -> Vec<T>;
}

impl<T> VecWithCloneableElementsExtension<T> for Vec<T>
where
    T: Clone,
{
    fn repeat_n(self, n: usize) -> itertools::RepeatN<Vec<T>> {
        itertools::repeat_n(self, n)
    }
    fn neighbor_indexes_around(&self, center_index: usize, radius: usize) -> RangeInclusive<usize> {
        let start_index = center_index.saturating_sub(radius);
        let end_index = center_index.add(radius).min(self.len());

        start_index..=end_index
    }
    fn repeat_first_element_at_end(mut self) -> Vec<T> {
        let first_element_clone = self.first().unwrap().clone();
        self.push(first_element_clone);
        self
    }
}
