use crate::prelude::*;

pub fn random_colors() -> RandomColorGenerator {
    RandomColorGenerator { colors: vec![] }
}

pub struct RandomColorGenerator {
    colors: Vec<Hsl>,
}

impl Generator<(), Hsl> for RandomColorGenerator {
    fn generate(&mut self, rand: &Rand, _input: ()) -> Hsl {
        if self.colors.is_empty() {
            panic!("No colors in the generator.");
        }

        *rand.element(&self.colors)
    }
}

impl RandomColorGenerator {
    pub fn color_picker(mut self, red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        let new_color = Colors::color_picker(red, green, blue, alpha);
        self.colors.push(new_color);
        self
    }
}
