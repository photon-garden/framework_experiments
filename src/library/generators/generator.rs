use crate::prelude::*;

pub type BoxedGenerator<T> = Box<dyn Generator<Output = T>>;

pub trait Generator {
    type Output;

    fn generate(&mut self, rand: &Rand) -> Self::Output;

    fn map<Mapper, NewOutput>(self, mapper: Mapper) -> MapGenerator<Self, Mapper, NewOutput>
    where
        Self: Sized,
        Mapper: Fn(Self::Output, &Rand) -> NewOutput,
    {
        MapGenerator {
            generator: self,
            mapper,
        }
    }
}
