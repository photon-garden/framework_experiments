use crate::prelude::*;

pub fn smart_looped_colors() -> SmartLoopedColorGenerator {
    SmartLoopedColorGenerator {
        colors: vec![],
        index: 0,
    }
}

pub struct SmartLoopedColorGenerator {
    colors: Vec<Hsl>,
    index: usize,
}

impl SmartGenerator<(), Hsl> for SmartLoopedColorGenerator {
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

impl SmartLoopedColorGenerator {
    pub fn color_picker(mut self, red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        let new_color = Colors::color_picker(red, green, blue, alpha);
        self.colors.push(new_color);
        self
    }
}
