use crate::prelude::*;
use std::marker::PhantomData;

type Mapper<OldOutput, NewOutput> = dyn Fn(OldOutput) -> NewOutput;

pub struct MapOutputSmartGenerator<SmartGen, Input, OldOutput, NewOutput>
where
    SmartGen: SmartGenerator<Input, OldOutput>,
{
    input: PhantomData<Input>,
    smart_generator: SmartGen,
    mapper: Box<Mapper<OldOutput, NewOutput>>,
}

impl<SmartGen, Input, OldOutput, NewOutput>
    MapOutputSmartGenerator<SmartGen, Input, OldOutput, NewOutput>
where
    SmartGen: SmartGenerator<Input, OldOutput>,
{
    pub fn new<Mapper>(smart_generator: SmartGen, mapper: Mapper) -> Self
    where
        Mapper: Fn(OldOutput) -> NewOutput + 'static,
    {
        Self {
            input: PhantomData,
            smart_generator,
            mapper: mapper.into_box(),
        }
    }
}

impl<SmartGen, Input, OldOutput, NewOutput> SmartGenerator<Input, NewOutput>
    for MapOutputSmartGenerator<SmartGen, Input, OldOutput, NewOutput>
where
    SmartGen: SmartGenerator<Input, OldOutput>,
{
    fn generate(&mut self, rand: &Rand, input: Input) -> NewOutput {
        let output = self.smart_generator.generate(rand, input);
        (self.mapper)(output)
    }
}
