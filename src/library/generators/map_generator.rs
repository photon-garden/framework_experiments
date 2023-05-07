use crate::prelude::*;

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
