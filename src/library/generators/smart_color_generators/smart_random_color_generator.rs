use crate::prelude::*;

pub fn smart_random_colors() -> SmartRandomColorGenerator {
    SmartRandomColorGenerator { colors: vec![] }
}

pub struct SmartRandomColorGenerator {
    colors: Vec<Hsl>,
}

impl SmartGenerator<(), Hsl> for SmartRandomColorGenerator {
    fn generate(&mut self, rand: &Rand, _input: ()) -> Hsl {
        if self.colors.is_empty() {
            panic!("No colors in the generator.");
        }

        *rand.element(&self.colors)
    }
}

impl SmartRandomColorGenerator {
    pub fn color_picker(mut self, red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        let new_color = Colors::color_picker(red, green, blue, alpha);
        self.colors.push(new_color);
        self
    }
}
