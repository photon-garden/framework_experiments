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

pub struct MapGenerator<Gen, Mapper, NewOutput>
where
    Gen: Generator,
    Mapper: Fn(Gen::Output, &Rand) -> NewOutput,
{
    generator: Gen,
    mapper: Mapper,
}

impl<Gen, Mapper, NewOutput> Generator for MapGenerator<Gen, Mapper, NewOutput>
where
    Gen: Generator,
    Mapper: Fn(Gen::Output, &Rand) -> NewOutput,
{
    type Output = NewOutput;

    fn generate(&mut self, rand: &Rand) -> Self::Output {
        let output = self.generator.generate(rand);
        (self.mapper)(output, rand)
    }
}
