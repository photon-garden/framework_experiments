use crate::prelude::*;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;

pub struct SaveOutputsGenerator<Gen, Input, Output>
where
    Gen: Generator<Input, Output>,
    Output: Clone,
{
    pub generator: Gen,
    pub outputs: Rc<RefCell<Vec<Output>>>,
    pub input: PhantomData<Input>,
}

impl<Gen, Input, Output> SaveOutputsGenerator<Gen, Input, Output>
where
    Gen: Generator<Input, Output>,
    Output: Clone,
{
    pub fn new(generator: Gen, outputs: Rc<RefCell<Vec<Output>>>) -> Self {
        Self {
            generator,
            outputs,
            input: PhantomData,
        }
    }
}

impl<Gen, Input, Output> Generator<Input, Output> for SaveOutputsGenerator<Gen, Input, Output>
where
    Gen: Generator<Input, Output>,
    Output: Clone,
{
    fn generate(&mut self, rand: &Rand, input: Input) -> Output {
        let output = self.generator.generate(rand, input);
        self.outputs.borrow_mut().push(output.clone());
        output
    }
}
