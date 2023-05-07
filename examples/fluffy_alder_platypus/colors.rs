use crate::prelude::*;

pub fn soft_white() -> Hsl {
    hsl(0.12466522, 0.34505126, 0.9302026)
}

pub fn soft_black() -> Hsl {
    Colors::color_picker(26, 26, 22, 255)
}

pub struct Colors {}

impl Colors {
    pub fn color_picker(red: u8, green: u8, blue: u8, alpha: u8) -> Hsl {
        srgba8(red, green, blue, alpha).into_hsl()
    }
}
