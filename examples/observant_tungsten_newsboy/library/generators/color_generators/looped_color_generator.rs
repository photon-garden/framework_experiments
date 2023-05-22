use crate::prelude::*;

pub fn looped_colors() -> LoopedColorGenerator {
    LoopedColorGenerator {
        colors: vec![],
        index: 0,
    }
}

pub struct LoopedColorGenerator {
    colors: Vec<Hsl>,
    index: usize,
}

impl Generator<(), Hsl> for LoopedColorGenerator {
    fn generate(&mut self, _rand: &Rand, _input: ()) -> Hsl {
        if self.colors.is_empty() {
            panic!("No colors in the generator.");
        }

        let looped_index = self.index.looped(self.colors.len());
        let next_color = self.colors[looped_index];
        self.index = looped_index + 1;
        next_color
    }
}

impl LoopedColorGenerator {
    pub fn color_picker(mut self, red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        let new_color = Colors::color_picker(red, green, blue, alpha);
        self.colors.push(new_color);
        self
    }
}
