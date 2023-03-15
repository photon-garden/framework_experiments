use crate::prelude::*;

pub trait OptionExtension<T> {
    fn is_some_and_matches<Predicate>(&self, predicate: Predicate) -> bool
    where
        Predicate: Fn(&T) -> bool;
}

impl<T> OptionExtension<T> for Option<T> {
    fn is_some_and_matches<Predicate>(&self, predicate: Predicate) -> bool
    where
        Predicate: Fn(&T) -> bool,
    {
        match self {
            Some(value) => predicate(&value),
            None => false,
        }
    }
}
