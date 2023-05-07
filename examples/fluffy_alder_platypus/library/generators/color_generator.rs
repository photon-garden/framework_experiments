use super::Generator;
use crate::prelude::*;
use std::iter::Cycle;

pub trait IntoBoxedColorGenerator {
    fn into_boxed_color_generator(self) -> Box<dyn Generator<Output = Hsl>>;
}

impl<T> IntoBoxedColorGenerator for T
where
    T: Generator<Output = Hsl> + 'static,
{
    fn into_boxed_color_generator(self) -> Box<dyn Generator<Output = Hsl>> {
        Box::new(self)
    }
}

pub struct OneColorGenerator {
    color: Hsl,
}

impl Generator for OneColorGenerator {
    type Output = Hsl;

    fn generate(&mut self, _rand: &Rand) -> Hsl {
        self.color
    }
}

impl OneColorGenerator {
    pub fn color_picker(mut self, red: u8, green: u8, blue: u8, alpha: u8) -> OneColorGenerator {
        let new_color = Colors::color_picker(red, green, blue, alpha);
        self.color = new_color;
        self
    }
}

pub struct RandomColors {
    colors: Vec<Hsl>,
}

impl Generator for RandomColors {
    type Output = Hsl;

    fn generate(&mut self, rand: &Rand) -> Hsl {
        if self.colors.is_empty() {
            panic!("No colors in the generator.");
        }

        *rand.element(&self.colors)
    }
}

impl RandomColors {
    pub fn color_picker(mut self, red: u8, green: u8, blue: u8, alpha: u8) -> RandomColors {
        let new_color = Colors::color_picker(red, green, blue, alpha);
        self.colors.push(new_color);
        self
    }
}

pub struct RepeatedColorGenerator {
    colors: Vec<Hsl>,
    index: usize,
}

impl Generator for RepeatedColorGenerator {
    type Output = Hsl;

    fn generate(&mut self, _rand: &Rand) -> Hsl {
        if self.colors.is_empty() {
            panic!("No colors in the generator.");
        }

        let looped_index = self.index.looped(self.colors.len());
        let next_color = self.colors[looped_index];
        self.index = looped_index + 1;
        next_color
    }
}

impl RepeatedColorGenerator {
    pub fn color_picker(
        mut self,
        red: u8,
        green: u8,
        blue: u8,
        alpha: u8,
    ) -> RepeatedColorGenerator {
        let new_color = Colors::color_picker(red, green, blue, alpha);
        self.colors.push(new_color);
        self
    }
}

pub fn color_picker(red: u8, green: u8, blue: u8, alpha: u8) -> OneColorGenerator {
    let color = crate::colors::Colors::color_picker(red, green, blue, alpha);
    OneColorGenerator { color }
}

pub fn random_colors() -> RandomColors {
    RandomColors { colors: vec![] }
}

pub fn repeated_colors() -> RepeatedColorGenerator {
    RepeatedColorGenerator {
        colors: vec![],
        index: 0,
    }
}

impl From<Hsl> for OneColorGenerator {
    fn from(color: Hsl) -> OneColorGenerator {
        OneColorGenerator { color }
    }
}
