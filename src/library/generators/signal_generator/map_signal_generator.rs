use crate::prelude::*;

pub struct MapSignalGenerator<SignalGen, Mapper, NewOutput>
where
    SignalGen: SignalGenerator,
    Mapper: Fn(SignalGen::Output, &Rand, NormalizedF32) -> NewOutput,
{
    signal_generator: SignalGen,
    mapper: Mapper,
}

impl<SignalGen, Mapper, NewOutput> SignalGenerator
    for MapSignalGenerator<SignalGen, Mapper, NewOutput>
where
    SignalGen: SignalGenerator,
    Mapper: Fn(SignalGen::Output, &Rand, NormalizedF32) -> NewOutput,
{
    type Output = NewOutput;

    fn generate(&mut self, rand: &Rand, progress: NormalizedF32) -> Self::Output {
        let output = self.signal_generator.generate(rand, progress);
        (self.mapper)(output, rand, progress)
    }
}
