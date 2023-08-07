use crate::prelude::*;
use std::marker::PhantomData;

type Mapper<OldOutput, NewOutput> = dyn Fn(OldOutput) -> NewOutput;

pub struct MapOutputGenerator<Gen, Input, OldOutput, NewOutput>
where
    Gen: Generator<Input, OldOutput>,
{
    input: PhantomData<Input>,
    generator: Gen,
    mapper: Box<Mapper<OldOutput, NewOutput>>,
}

impl<Gen, Input, OldOutput, NewOutput> MapOutputGenerator<Gen, Input, OldOutput, NewOutput>
where
    Gen: Generator<Input, OldOutput>,
{
    pub fn new<Mapper>(generator: Gen, mapper: Mapper) -> Self
    where
        Mapper: Fn(OldOutput) -> NewOutput + 'static,
    {
        Self {
            input: PhantomData,
            generator,
            mapper: mapper.into_box(),
        }
    }
}

impl<Gen, Input, OldOutput, NewOutput> Generator<Input, NewOutput>
    for MapOutputGenerator<Gen, Input, OldOutput, NewOutput>
where
    Gen: Generator<Input, OldOutput>,
{
    fn generate(&mut self, rand: &Rand, input: Input) -> NewOutput {
        let output = self.generator.generate(rand, input);
        (self.mapper)(output)
    }
}
