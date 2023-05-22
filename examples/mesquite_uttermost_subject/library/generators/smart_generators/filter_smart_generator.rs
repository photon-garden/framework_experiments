use crate::prelude::*;
use std::marker::PhantomData;

pub struct FilterSmartGenerator<SmartGen, Filter, Input, Output>
where
    SmartGen: SmartGenerator<Input, Output>,
    Filter: Fn(&SmartGen, &Rand, &Input, &Output) -> bool,
{
    generator: SmartGen,
    filter: Filter,
    input: PhantomData<Input>,
    output: PhantomData<Output>,
}

impl<SmartGen, Filter, Input, Output> FilterSmartGenerator<SmartGen, Filter, Input, Output>
where
    SmartGen: SmartGenerator<Input, Output>,
    Filter: Fn(&SmartGen, &Rand, &Input, &Output) -> bool,
{
    pub fn new(generator: SmartGen, filter: Filter) -> Self {
        Self {
            generator,
            filter,
            input: PhantomData,
            output: PhantomData,
        }
    }
}

impl<SmartGen, Filter, Input, Output> SmartGenerator<Input, Output>
    for FilterSmartGenerator<SmartGen, Filter, Input, Output>
where
    SmartGen: SmartGenerator<Input, Output>,
    Filter: Fn(&SmartGen, &Rand, &Input, &Output) -> bool,
    Input: Clone,
{
    fn generate(&mut self, rand: &Rand, input: Input) -> Output {
        let num_tries = 1_000;
        for _ in 0..num_tries {
            let output = self.generator.generate(rand, input.clone());
            let should_keep = (self.filter)(&self.generator, rand, &input, &output);
            if should_keep {
                return output;
            }
        }

        panic!(
            "FilterSmartGenerator.generator.generate() failed to generate a value that passes the filter after {num_tries} iterations."
        );
    }
}
