use crate::prelude::*;

pub trait IteratorExtension<AnIterator, AnItem>
where
    AnIterator: Iterator<Item = AnItem>,
{
    fn take_to_vec(&mut self, num_elements: usize) -> Vec<AnItem>;
}

impl<AnIterator, AnItem> IteratorExtension<AnIterator, AnItem> for AnIterator
where
    AnIterator: Iterator<Item = AnItem>,
{
    fn take_to_vec(&mut self, num_elements: usize) -> Vec<AnItem> {
        let mut items = Vec::with_capacity(num_elements);

        for _ in 0..num_elements {
            match self.next() {
                Some(next_item) => {
                    items.push(next_item);
                }
                None => break,
            }
        }

        items
    }
}
