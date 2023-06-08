use crate::prelude::*;

pub fn context_uniform_random_bool(probability_of_true: NormalizedF32) -> ContextUniformRandomBool {
    ContextUniformRandomBool {
        probability_of_true,
    }
}

pub struct ContextUniformRandomBool {
    probability_of_true: NormalizedF32,
}

impl<Context> GeneratorHeart<(), bool, Context> for ContextUniformRandomBool
where
    Context: Sized + 'static,
{
    fn generate_with_context(&mut self, params: &GenerateWithContextParams<(), Context>) -> bool {
        params.rand.flip_coin(self.probability_of_true)
    }
}

impl IntoContextGenerator<(), bool> for ContextUniformRandomBool {
    fn into_context_generator(self) -> ContextGenerator<(), bool> {
        self.without_context().into_context_generator()
    }
}
