use std::iter::Cycle;

use crate::prelude::*;

pub struct ColorGenerator {
    mode: ColorGeneratorMode,
}

impl ColorGenerator {
    pub fn generate(&mut self, rand: &Rand) -> Hsl {
        match &mut self.mode {
            ColorGeneratorMode::OneColor(color) => *color,

            ColorGeneratorMode::Random(colors) => {
                if colors.is_empty() {
                    panic!("No colors in the generator.");
                }

                *rand.element(&colors)
            }

            ColorGeneratorMode::Repeated(colors, index) => {
                if colors.is_empty() {
                    panic!("No colors in the generator.");
                }

                let looped_index = index.looped(colors.len());
                let next_color = colors[looped_index];
                *index = looped_index + 1;
                next_color
            }
        }
    }

    pub fn color_picker(mut self, red: u8, green: u8, blue: u8, alpha: u8) -> ColorGenerator {
        let new_color = Colors::color_picker(red, green, blue, alpha);

        match &mut self.mode {
            ColorGeneratorMode::OneColor(_) => {
                self.mode = ColorGeneratorMode::OneColor(new_color);
            }

            ColorGeneratorMode::Random(colors) => {
                colors.push(new_color);
            }

            ColorGeneratorMode::Repeated(colors, _) => {
                colors.push(new_color);
            }
        }

        self
    }
}

pub enum ColorGeneratorMode {
    OneColor(Hsl),
    Random(Vec<Hsl>),
    Repeated(Vec<Hsl>, usize),
}

pub fn color_picker(red: u8, green: u8, blue: u8, alpha: u8) -> ColorGenerator {
    let color = Colors::color_picker(red, green, blue, alpha);
    ColorGenerator {
        mode: ColorGeneratorMode::OneColor(color),
    }
}

pub fn random_colors() -> ColorGenerator {
    ColorGenerator {
        mode: ColorGeneratorMode::Random(vec![]),
    }
}

pub fn repeated_colors() -> ColorGenerator {
    ColorGenerator {
        mode: ColorGeneratorMode::Repeated(vec![], 0),
    }
}

impl Into<ColorGenerator> for Hsl {
    fn into(self) -> ColorGenerator {
        ColorGenerator {
            mode: ColorGeneratorMode::OneColor(self),
        }
    }
}
