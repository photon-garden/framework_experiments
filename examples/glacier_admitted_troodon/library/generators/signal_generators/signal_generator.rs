use crate::prelude::*;

pub type BoxedSignalGenerator<T> = Box<dyn SignalGenerator<Output = T>>;

pub trait SignalGenerator {
    type Output;

    // Could we be more generic here? Instead of a progress argument, maybe
    // it could be some kind of generic context argument?
    fn generate(&mut self, rand: &Rand, progress: NormalizedF32) -> Self::Output;

    fn map<Mapper, NewOutput>(self, mapper: Mapper) -> MapSignalGenerator<Self, NewOutput>
    where
        Self: Sized,
        Mapper: Fn(Self::Output, &Rand, NormalizedF32) -> NewOutput + 'static,
    {
        MapSignalGenerator::new(self, mapper)
    }

    fn crystallize(self) -> CrystallizedSignalGenerator<Self>
    where
        Self: Sized,
        Self::Output: Clone,
    {
        CrystallizedSignalGenerator::new(self)
    }
}

// Any regular generator can be used as a signal generator. Just ignore the progress argument.
impl<Gen> SignalGenerator for Gen
where
    Gen: Generator,
{
    type Output = Gen::Output;

    fn generate(&mut self, rand: &Rand, _progress: NormalizedF32) -> Self::Output {
        self.generate(rand)
    }
}
