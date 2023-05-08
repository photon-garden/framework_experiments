use crate::prelude::*;

pub type BoxedSmartGenerator<In, Out> = Box<dyn SmartGenerator<In, Out>>;
pub type SmartSignalGenerator<Out> = BoxedSmartGenerator<NormalizedF32, Out>;
pub type SmartSimpleGenerator<Out> = BoxedSmartGenerator<(), Out>;

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
}

pub trait IntoSmartGenerator<In, Out> {
    fn into_smart_generator(self) -> BoxedSmartGenerator<In, Out>;
}

impl<SmartGen, In, Out> IntoSmartGenerator<In, Out> for SmartGen
where
    SmartGen: SmartGenerator<In, Out> + 'static,
{
    fn into_smart_generator(self) -> BoxedSmartGenerator<In, Out> {
        Box::new(self)
    }
}

impl<In, Out> SmartGenerator<In, Out> for fn(&Rand, In) -> Out {
    fn generate(&mut self, rand: &Rand, input: In) -> Out {
        (self)(rand, input)
    }
}

impl<In, Out> SmartGenerator<In, Out> for Out
where
    Out: CanBeAutomaticallyConvertedToSmartGenerator,
{
    fn generate(&mut self, _rand: &Rand, _input: In) -> Out {
        self.clone()
    }
}

trait CanBeAutomaticallyConvertedToSmartGenerator: Clone {}

impl CanBeAutomaticallyConvertedToSmartGenerator for f32 {}
