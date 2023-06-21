use crate::prelude::*;

pub fn context_looped_hsl() -> ContextLoopedHsl {
    ContextLoopedHsl {
        colors: vec![],
        index: 0,
    }
}

pub struct ContextLoopedHsl {
    colors: Vec<Hsl>,
    index: usize,
}

impl ContextLoopedHsl {
    pub fn color_picker(mut self, red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        let new_color = Colors::color_picker(red, green, blue, alpha);
        self.colors.push(new_color);
        self
    }
}

impl<Context> GeneratorHeart<(), Hsl, Context> for ContextLoopedHsl
where
    Context: Sized + 'static,
{
    fn generate_with_context(&mut self, _params: &GenerateWithContextParams<(), Context>) -> Hsl {
        if self.colors.is_empty() {
            panic!("ContextLoopedHsl didn't get any colors in the generator heart.");
        }

        let looped_index = self.index.looped(self.colors.len());
        let next_color = self.colors[looped_index];
        self.index = looped_index + 1;
        next_color
    }
}

impl IntoContextGenerator<(), Hsl> for ContextLoopedHsl {
    fn into_context_generator(self) -> ContextGenerator<(), Hsl> {
        self.without_context().into_context_generator()
    }
}
