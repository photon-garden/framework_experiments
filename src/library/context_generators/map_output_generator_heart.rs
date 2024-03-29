use crate::prelude::*;
use std::marker::PhantomData;

type Mapper<OldOutput, NewOutput> = dyn Fn(OldOutput) -> NewOutput;

pub struct MapOutputGeneratorHeart<Gen, Input, OldOutput, NewOutput, Context>
where
    Input: 'static,
    OldOutput: Clone + 'static,
    NewOutput: Clone + 'static,
    Context: Sized + 'static,
    Gen: GeneratorHeart<Input, OldOutput, Context>,
{
    input: PhantomData<Input>,
    context: PhantomData<Context>,
    generator: Gen,
    mapper: Box<Mapper<OldOutput, NewOutput>>,
}

impl<Gen, Input, OldOutput, NewOutput, Context>
    MapOutputGeneratorHeart<Gen, Input, OldOutput, NewOutput, Context>
where
    Input: 'static,
    OldOutput: Clone + 'static,
    NewOutput: Clone + 'static,
    Context: Sized + 'static,
    Gen: GeneratorHeart<Input, OldOutput, Context>,
{
    pub fn new<Mapper>(generator: Gen, mapper: Mapper) -> Self
    where
        Mapper: Fn(OldOutput) -> NewOutput + 'static,
    {
        Self {
            input: PhantomData,
            context: PhantomData,
            generator,
            mapper: mapper.into_box(),
        }
    }
}

impl<Gen, Input, OldOutput, NewOutput, Context> GeneratorHeart<Input, NewOutput, Context>
    for MapOutputGeneratorHeart<Gen, Input, OldOutput, NewOutput, Context>
where
    Input: 'static,
    OldOutput: Clone + 'static,
    NewOutput: Clone + 'static,
    Context: Sized + 'static,
    Gen: GeneratorHeart<Input, OldOutput, Context>,
{
    fn generate_with_context(
        &mut self,
        params: &GenerateWithContextParams<Input, Context>,
    ) -> NewOutput {
        let output = self.generator.generate_with_context(params);
        (self.mapper)(output)
    }
}

impl<Gen, Input, OldOutput, NewOutput> IntoContextGenerator<Input, NewOutput>
    for MapOutputGeneratorHeart<Gen, Input, OldOutput, NewOutput, ()>
where
    Input: 'static,
    OldOutput: Clone + 'static,
    NewOutput: Clone + 'static,
    Gen: GeneratorHeart<Input, OldOutput, ()> + 'static,
{
    fn into_context_generator(self) -> ContextGenerator<Input, NewOutput> {
        self.without_context().into_context_generator()
    }
}
