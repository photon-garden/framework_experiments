use crate::prelude::*;
use std::marker::PhantomData;

pub struct FilterGenerator<Gen, Filter, Input, Output>
where
    Gen: Generator<Input, Output>,
    Filter: Fn(FilterParams<Gen, Input, Output>) -> bool,
{
    generator: Gen,
    filter: Filter,
    input: PhantomData<Input>,
    output: PhantomData<Output>,
}

impl<Gen, Filter, Input, Output> FilterGenerator<Gen, Filter, Input, Output>
where
    Gen: Generator<Input, Output>,
    Filter: Fn(FilterParams<Gen, Input, Output>) -> bool,
{
    pub fn new(generator: Gen, filter: Filter) -> Self {
        Self {
            generator,
            filter,
            input: PhantomData,
            output: PhantomData,
        }
    }
}

impl<Gen, Filter, Input, Output> Generator<Input, Output>
    for FilterGenerator<Gen, Filter, Input, Output>
where
    Gen: Generator<Input, Output>,
    Filter: Fn(FilterParams<Gen, Input, Output>) -> bool,
    Input: Clone,
{
    fn generate(&mut self, rand: &Rand, input: Input) -> Output {
        let num_tries = 1_000;
        for _ in 0..num_tries {
            let output = self.generator.generate(rand, input.clone());
            let filter_params = FilterParams {
                generator: &self.generator,
                input: &input,
                output: &output,
                rand,
            };
            let should_keep = (self.filter)(filter_params);
            if should_keep {
                return output;
            }
        }

        panic!(
            "FilterGenerator.generator.generate() failed to generate a value that passes the filter after {num_tries} iterations."
        );
    }
}

pub struct FilterParams<'a, Gen, Input, Output>
where
    Gen: Generator<Input, Output>,
{
    pub generator: &'a Gen,
    pub input: &'a Input,
    pub output: &'a Output,
    pub rand: &'a Rand,
}
