use crate::prelude::*;

type Mapper<Input, Output> = dyn Fn(Input, &Rand, NormalizedF32) -> Output;

pub struct MapSignalGenerator<SignalGen, NewOutput>
where
    SignalGen: SignalGenerator,
{
    signal_generator: SignalGen,
    mapper: Box<Mapper<SignalGen::Output, NewOutput>>,
}

impl<SignalGen, NewOutput> MapSignalGenerator<SignalGen, NewOutput>
where
    SignalGen: SignalGenerator,
{
    pub fn new<Mapper>(signal_generator: SignalGen, mapper: Mapper) -> Self
    where
        Mapper: Fn(SignalGen::Output, &Rand, NormalizedF32) -> NewOutput + 'static,
    {
        Self {
            signal_generator,
            mapper: mapper.into_box(),
        }
    }
}

impl<SignalGen, NewOutput> SignalGenerator for MapSignalGenerator<SignalGen, NewOutput>
where
    SignalGen: SignalGenerator,
{
    type Output = NewOutput;

    fn generate(&mut self, rand: &Rand, progress: NormalizedF32) -> Self::Output {
        let output = self.signal_generator.generate(rand, progress);
        (self.mapper)(output, rand, progress)
    }
}
