use crate::prelude::*;

pub type BoxedSmartGenerator<In, Out> = Box<dyn SmartGenerator<In, Out>>;

pub trait SmartGenerator<Input, Output> {
    fn generate(&mut self, rand: &Rand, input: Input) -> Output;

    fn map_input<Mapper, NewInput>(
        self,
        mapper: Mapper,
    ) -> MapInputSmartGenerator<Self, Input, NewInput, Output>
    where
        Self: Sized,
        Mapper: Fn(NewInput) -> Input + 'static,
    {
        MapInputSmartGenerator::new(self, mapper)
    }

    fn map_output<Mapper, NewOutput>(
        self,
        mapper: Mapper,
    ) -> MapOutputSmartGenerator<Self, Input, Output, NewOutput>
    where
        Self: Sized,
        Mapper: Fn(Output) -> NewOutput + 'static,
    {
        MapOutputSmartGenerator::new(self, mapper)
    }

    fn filter<Filter>(self, filter: Filter) -> FilterSmartGenerator<Self, Filter, Input, Output>
    where
        Self: Sized,
        Filter: Fn(FilterParams<Self, Input, Output>) -> bool,
    {
        FilterSmartGenerator::new(self, filter)
    }
}

pub trait IntoSmartGenerator<In, Out>: SmartGenerator<In, Out> + 'static + Sized {
    fn into_smart_generator(self) -> BoxedSmartGenerator<In, Out> {
        Box::new(self)
    }
}

impl<In, Out, SmartGen> IntoSmartGenerator<In, Out> for SmartGen where
    SmartGen: SmartGenerator<In, Out> + 'static
{
}

impl<Function, In, Out> SmartGenerator<In, Out> for Function
where
    Function: Fn(&Rand, In) -> Out,
{
    fn generate(&mut self, rand: &Rand, input: In) -> Out {
        (self)(rand, input)
    }
}

impl<In> SmartGenerator<In, f32> for f32 {
    fn generate(&mut self, _rand: &Rand, _input: In) -> f32 {
        *self
    }
}
