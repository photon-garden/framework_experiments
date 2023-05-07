use crate::prelude::*;
use nannou::color::{Hsla, IntoLinSrgba};

pub trait Srgba8Extension {
    fn into_hsla(self) -> Hsla;
}

impl Srgba8Extension for Srgba<u8> {
    fn into_hsla(self) -> Hsla {
        self.into_lin_srgba().into()
    }
}
