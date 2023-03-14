use crate::prelude::*;

pub trait IterF32Extension {
    fn average(&mut self) -> f32;
}

impl<T> IterF32Extension for T
where
    T: Iterator<Item = f32>,
{
    fn average(&mut self) -> f32 {
        let mut total = 0.0;
        let mut num_elements = 0;
        while let Some(value) = self.next() {
            num_elements += 1;
            total += value;
        }
        total / num_elements as f32
    }
}
