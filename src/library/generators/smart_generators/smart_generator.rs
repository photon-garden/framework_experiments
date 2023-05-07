use crate::prelude::*;

pub type BoxedSmartGenerator<In, Out> = Box<dyn SmartGenerator<Input = In, Output = Out>>;
pub type SmartSignalGenerator<Out> = BoxedSmartGenerator<NormalizedF32, Out>;
pub type SmartSimpleGenerator<Out> = BoxedSmartGenerator<(), Out>;

pub trait SmartGenerator {
    type Input;
    type Output;

    fn generate(&mut self, rand: &Rand, input: Self::Input) -> Self::Output;
}

pub trait IntoSmartGenerator<In, Out> {
    fn into_smart_generator(self) -> BoxedSmartGenerator<In, Out>;
}

impl<F, In, Out> IntoSmartGenerator<In, Out> for F
where
    F: Fn(&Rand, In) -> Out + 'static,
    In: 'static,
    Out: 'static,
{
    fn into_smart_generator(self) -> BoxedSmartGenerator<In, Out> {
        Box::new(FunctionSmartGenerator {
            function: Box::new(self),
        })
    }
}

struct FunctionSmartGenerator<In, Out> {
    function: Box<dyn Fn(&Rand, In) -> Out>,
}

impl<In, Out> SmartGenerator for FunctionSmartGenerator<In, Out> {
    type Input = In;
    type Output = Out;

    fn generate(&mut self, rand: &Rand, input: Self::Input) -> Self::Output {
        (self.function)(rand, input)
    }
}
