use crate::prelude::*;

pub type BoxedGenerator<In, Out> = Box<dyn Generator<In, Out>>;

pub trait Generator<Input, Output> {
    fn generate(&mut self, rand: &Rand, input: Input) -> Output;

    fn map_input<Mapper, NewInput>(
        self,
        mapper: Mapper,
    ) -> MapInputGenerator<Self, Input, NewInput, Output>
    where
        Self: Sized,
        Mapper: Fn(NewInput) -> Input + 'static,
    {
        MapInputGenerator::new(self, mapper)
    }

    fn map_output<Mapper, NewOutput>(
        self,
        mapper: Mapper,
    ) -> MapOutputGenerator<Self, Input, Output, NewOutput>
    where
        Self: Sized,
        Mapper: Fn(Output) -> NewOutput + 'static,
    {
        MapOutputGenerator::new(self, mapper)
    }

    fn filter<Filter>(self, filter: Filter) -> FilterGenerator<Self, Filter, Input, Output>
    where
        Self: Sized,
        Filter: Fn(FilterParams<Self, Input, Output>) -> bool,
    {
        FilterGenerator::new(self, filter)
    }

    fn crystallize(self) -> CrystallizedGenerator<Self, Input, Output>
    where
        Self: Sized,
        Output: Clone,
    {
        CrystallizedGenerator::new(self)
    }
}

pub trait IntoGenerator<In, Out>: Generator<In, Out> + 'static + Sized {
    fn into_generator(self) -> BoxedGenerator<In, Out> {
        Box::new(self)
    }
}

impl<In, Out, Gen> IntoGenerator<In, Out> for Gen where Gen: Generator<In, Out> + 'static {}

impl<Function, In, Out> Generator<In, Out> for Function
where
    Function: Fn(&Rand, In) -> Out,
{
    fn generate(&mut self, rand: &Rand, input: In) -> Out {
        (self)(rand, input)
    }
}

// impl<Gen, Input, Output> Generator<Input, Output> for Gen
// where
//     Gen: Generator<(), Output>,
// {
//     fn generate(&mut self, rand: &Rand, _input: Input) -> Output {
//         self.generate(rand, ())
//     }
// }
