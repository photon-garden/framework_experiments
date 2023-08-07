use crate::prelude::*;

pub fn looped_hsl() -> LoopedHsl {
    LoopedHsl {
        colors: vec![],
        index: 0,
    }
}

pub struct LoopedHsl {
    colors: Vec<Hsl>,
    index: usize,
}

impl LoopedHsl {
    pub fn color_picker(mut self, red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        let new_color = Colors::color_picker(red, green, blue, alpha);
        self.colors.push(new_color);
        self
    }
}

impl<Context> GeneratorHeart<(), Hsl, Context> for LoopedHsl
where
    Context: Sized + 'static,
{
    fn generate_with_context(&mut self, _params: &GenerateWithContextParams<(), Context>) -> Hsl {
        if self.colors.is_empty() {
            panic!("LoopedHsl didn't get any colors in the generator heart.");
        }

        let looped_index = self.index.looped(self.colors.len());
        let next_color = self.colors[looped_index];
        self.index = looped_index + 1;
        next_color
    }
}

impl IntoContextGenerator<(), Hsl> for LoopedHsl {
    fn into_context_generator(self) -> ContextGenerator<(), Hsl> {
        self.without_context().into_context_generator()
    }
}
