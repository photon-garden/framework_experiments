use crate::prelude::*;
use std::marker::PhantomData;

type Mapper<NewInput, OldInput> = Box<dyn Fn(NewInput) -> OldInput>;

pub struct MapInputSmartGenerator<SmartGen, OldInput, NewInput, Output>
where
    SmartGen: SmartGenerator<OldInput, Output>,
{
    output: PhantomData<Output>,
    smart_generator: SmartGen,
    mapper: Mapper<NewInput, OldInput>,
}

impl<SmartGen, OldInput, NewInput, Output>
    MapInputSmartGenerator<SmartGen, OldInput, NewInput, Output>
where
    SmartGen: SmartGenerator<OldInput, Output>,
{
    pub fn new<Mapper>(smart_generator: SmartGen, mapper: Mapper) -> Self
    where
        Mapper: Fn(NewInput) -> OldInput + 'static,
    {
        Self {
            output: PhantomData,
            smart_generator,
            mapper: mapper.into_box(),
        }
    }
}

impl<SmartGen, OldInput, NewInput, Output> SmartGenerator<NewInput, Output>
    for MapInputSmartGenerator<SmartGen, OldInput, NewInput, Output>
where
    SmartGen: SmartGenerator<OldInput, Output>,
{
    fn generate(&mut self, rand: &Rand, input: NewInput) -> Output {
        let mapped_input = (self.mapper)(input);
        self.smart_generator.generate(rand, mapped_input)
    }
}
