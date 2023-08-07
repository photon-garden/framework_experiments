use crate::prelude::*;

pub fn context_uniform_random_bool(probability_of_true: NormalizedF32) -> UniformRandomBool {
    UniformRandomBool {
        probability_of_true,
    }
}

pub struct UniformRandomBool {
    probability_of_true: NormalizedF32,
}

impl<Context> GeneratorHeart<(), bool, Context> for UniformRandomBool
where
    Context: Sized + 'static,
{
    fn generate_with_context(&mut self, params: &GenerateWithContextParams<(), Context>) -> bool {
        params.rand.flip_coin(self.probability_of_true)
    }
}

impl IntoContextGenerator<(), bool> for UniformRandomBool {
    fn into_context_generator(self) -> ContextGenerator<(), bool> {
        self.without_context().into_context_generator()
    }
}
