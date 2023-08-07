use crate::prelude::*;
use std::marker::PhantomData;

type Mapper<NewInput, OldInput> = dyn Fn(&NewInput) -> OldInput;

pub struct MapInputGeneratorHeart<Gen, OldInput, NewInput, Output, Context>
where
    OldInput: 'static,
    NewInput: 'static,
    Output: Clone + 'static,
    Context: Sized + 'static,
    Gen: GeneratorHeart<OldInput, Output, Context>,
{
    output: PhantomData<Output>,
    context: PhantomData<Context>,
    generator: Gen,
    mapper: Box<Mapper<NewInput, OldInput>>,
}

impl<Gen, OldInput, NewInput, Output, Context>
    MapInputGeneratorHeart<Gen, OldInput, NewInput, Output, Context>
where
    OldInput: 'static,
    NewInput: 'static,
    Output: Clone + 'static,
    Context: Sized + 'static,
    Gen: GeneratorHeart<OldInput, Output, Context>,
{
    pub fn new<Mapper>(generator: Gen, mapper: Mapper) -> Self
    where
        Mapper: Fn(&NewInput) -> OldInput + 'static,
    {
        Self {
            output: PhantomData,
            context: PhantomData,
            generator,
            mapper: mapper.into_box(),
        }
    }
}

impl<Gen, OldInput, NewInput, Output, Context> GeneratorHeart<NewInput, Output, Context>
    for MapInputGeneratorHeart<Gen, OldInput, NewInput, Output, Context>
where
    OldInput: 'static,
    NewInput: 'static,
    Output: Clone + 'static,
    Context: Sized + 'static,
    Gen: GeneratorHeart<OldInput, Output, Context>,
{
    fn generate_with_context(
        &mut self,
        params: &GenerateWithContextParams<NewInput, Context>,
    ) -> Output {
        let new_input = (self.mapper)(params.input);
        let new_params = GenerateWithContextParams {
            input: &new_input,
            context: params.context,
            rand: params.rand,
        };

        self.generator.generate_with_context(&new_params)
    }
}
