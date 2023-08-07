use crate::prelude::*;
use std::marker::PhantomData;

type Mapper<NewInput, OldInput> = Box<dyn Fn(NewInput) -> OldInput>;

pub struct MapInputGenerator<Gen, OldInput, NewInput, Output>
where
    Gen: Generator<OldInput, Output>,
{
    output: PhantomData<Output>,
    generator: Gen,
    mapper: Mapper<NewInput, OldInput>,
}

impl<Gen, OldInput, NewInput, Output> MapInputGenerator<Gen, OldInput, NewInput, Output>
where
    Gen: Generator<OldInput, Output>,
{
    pub fn new<Mapper>(generator: Gen, mapper: Mapper) -> Self
    where
        Mapper: Fn(NewInput) -> OldInput + 'static,
    {
        Self {
            output: PhantomData,
            generator,
            mapper: mapper.into_box(),
        }
    }
}

impl<Gen, OldInput, NewInput, Output> Generator<NewInput, Output>
    for MapInputGenerator<Gen, OldInput, NewInput, Output>
where
    Gen: Generator<OldInput, Output>,
{
    fn generate(&mut self, rand: &Rand, input: NewInput) -> Output {
        let mapped_input = (self.mapper)(input);
        self.generator.generate(rand, mapped_input)
    }
}
