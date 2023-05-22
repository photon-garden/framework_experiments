use crate::prelude::*;

type Mapper<Input, Output> = dyn Fn(Input, &Rand) -> Output;

pub struct MapGenerator<Gen, NewOutput>
where
    Gen: Generator,
{
    generator: Gen,
    mapper: Box<Mapper<Gen::Output, NewOutput>>,
}

impl<Gen, NewOutput> MapGenerator<Gen, NewOutput>
where
    Gen: Generator,
{
    pub fn new<Mapper>(generator: Gen, mapper: Mapper) -> Self
    where
        Mapper: Fn(Gen::Output, &Rand) -> NewOutput + 'static,
    {
        Self {
            generator,
            mapper: mapper.into_box(),
        }
    }
}

impl<Gen, NewOutput> Generator for MapGenerator<Gen, NewOutput>
where
    Gen: Generator,
{
    type Output = NewOutput;

    fn generate(&mut self, rand: &Rand) -> Self::Output {
        let output = self.generator.generate(rand);
        (self.mapper)(output, rand)
    }
}
