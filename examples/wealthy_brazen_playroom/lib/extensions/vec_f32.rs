use crate::prelude::*;

pub trait VecF32Extension {
    fn normalize(&self) -> Self;
    fn denormalize(&self, output_min: f32, output_max: f32) -> Vec<f32>;
}

impl VecF32Extension for Vec<f32> {
    fn normalize(&self) -> Self {
        let first = self.get(0);

        if first.is_none() {
            return vec![];
        }

        let mut min = first.unwrap();
        let mut max = first.unwrap();

        for number in self.iter() {
            if number < min {
                min = number;
            }

            if number > max {
                max = number;
            }
        }

        self.iter()
            .map(|number| (*number).normalize(*min, *max))
            .collect()
    }
    fn denormalize(&self, output_min: f32, output_max: f32) -> Vec<f32> {
        self.iter()
            .map(|number| number.denormalize(output_min, output_max))
            .collect()
    }
}
