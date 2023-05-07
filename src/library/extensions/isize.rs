use crate::prelude::*;

pub trait ISizeExtension {
    fn minus(&self, other: isize) -> isize;
}

impl ISizeExtension for isize {
    fn minus(&self, other: isize) -> isize {
        self - other
    }
}
