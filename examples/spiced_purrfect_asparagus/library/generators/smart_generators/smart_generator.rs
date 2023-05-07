pub type BoxedSmartGenerator<I, O> = Box<dyn SmartGenerator<Input = I, Output = O>>;

pub trait SmartGenerator {
    type Input;
    type Output;

    fn generate(&mut self, input: Self::Input) -> Self::Output;
}

pub trait IntoSmartGenerator<I, O> {
    fn into_smart_generator(self) -> BoxedSmartGenerator<I, O>;
}

impl<F, I, O> IntoSmartGenerator<I, O> for F
where
    F: Fn(I) -> O + 'static,
    I: 'static,
    O: 'static,
{
    fn into_smart_generator(self) -> BoxedSmartGenerator<I, O> {
        Box::new(FunctionSmartGenerator {
            function: Box::new(self),
        })
    }
}

struct FunctionSmartGenerator<I, O> {
    function: Box<dyn Fn(I) -> O>,
}

impl<I, O> SmartGenerator for FunctionSmartGenerator<I, O> {
    type Input = I;
    type Output = O;

    fn generate(&mut self, input: Self::Input) -> Self::Output {
        (self.function)(input)
    }
}
