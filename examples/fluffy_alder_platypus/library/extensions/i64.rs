use crate::prelude::*;

pub trait I64Extension {
    fn minus(&self, other: i64) -> i64;
}

impl I64Extension for i64 {
    fn minus(&self, other: i64) -> i64 {
        self - other
    }
}
