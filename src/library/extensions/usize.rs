use crate::prelude::*;

pub trait USizeExtension {
    fn is_even(&self) -> bool;
    fn is_odd(&self) -> bool;
    fn floor_to_odd(&self) -> usize;
    fn is_divisible_by(&self, other_number: usize) -> bool;
    fn normalize(&self, min: usize, max: usize) -> f32;
    fn as_f32(&self) -> f32;
    fn minus(&self, other: usize) -> usize;
    fn plus(&self, other: usize) -> usize;
    fn times(&self, other: usize) -> usize;
    fn divided_by(&self, other: usize) -> usize;
    fn looped(&self, loop_size: usize) -> usize;
}

impl USizeExtension for usize {
    fn is_divisible_by(&self, other_number: usize) -> bool {
        self % other_number == 0
    }
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
    fn is_odd(&self) -> bool {
        self % 2 != 0
    }
    fn floor_to_odd(&self) -> usize {
        if self.is_even() { self - 1 } else { *self }
    }
    fn normalize(&self, min: usize, max: usize) -> f32 {
        let self_f32 = *self as f32;
        let min_f32 = min as f32;
        let max_f32 = max as f32;

        self_f32.normalize(min_f32, max_f32)
    }

    fn as_f32(&self) -> f32 {
        *self as f32
    }

    fn minus(&self, other: usize) -> usize {
        self - other
    }

    fn plus(&self, other: usize) -> usize {
        self + other
    }

    fn times(&self, other: usize) -> usize {
        self * other
    }

    fn divided_by(&self, other: usize) -> usize {
        self / other
    }

    fn looped(&self, loop_size: usize) -> usize {
        *self % loop_size
    }
}
