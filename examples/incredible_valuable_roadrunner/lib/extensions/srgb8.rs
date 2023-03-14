use crate::prelude::*;
use nannou::color::{Hsl, IntoLinSrgba, Srgb};

pub trait Srgb8Extension {
    fn into_hsl(self) -> Hsl;
    fn mix_pigment(&self, other: &Srgb<u8>, progress: f32) -> Srgb<u8>;
}

impl Srgb8Extension for Srgb<u8> {
    fn into_hsl(self) -> Hsl {
        self.into_lin_srgba().into()
    }

    fn mix_pigment(&self, other: &Srgb<u8>, progress: f32) -> Srgb<u8> {
        let [red, green, blue] =
            pigment_mixing::mix_srgb_u8(self.as_ref(), other.as_ref(), progress);
        srgb8(red, green, blue)
    }
}
