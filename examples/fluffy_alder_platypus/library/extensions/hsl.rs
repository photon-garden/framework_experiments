use crate::prelude::*;
use nannou::color::Hue;
use nannou::color::Saturate;
use nannou::color::Shade;
use nannou::color::{FromColor, Hsl, Srgb};

pub trait HslExtension {
    fn to_code(&self) -> String;
    fn lerp(&self, other: &Hsl, progress: f32) -> Hsl;
    fn normalized_hue(&self) -> NormalizedF32;
    fn mix_pigment(&self, other: &Hsl, progress: f32) -> Hsl;
    fn as_srgb8(&self) -> Srgb<u8>;
    fn interpolated_palette(&self, other: &Hsl, num_steps: usize) -> Vec<Hsl>;
    fn jitter_hue(&self, rand: &Rand, amount: f32) -> Hsl;
    fn jitter_saturation(&self, rand: &Rand, amount: f32) -> Hsl;
    fn jitter_lightness(&self, rand: &Rand, amount: f32) -> Hsl;
}

impl HslExtension for Hsl {
    fn normalized_hue(&self) -> NormalizedF32 {
        self.hue.to_normalized()
    }
    fn to_code(&self) -> String {
        let hue = self.normalized_hue();
        format!(
            "let color = hsl({}, {}, {});",
            hue, self.saturation, self.lightness
        )
    }
    fn lerp(&self, other: &Hsl, progress: f32) -> Hsl {
        let hue = progress.linear_interpolate(self.normalized_hue(), other.normalized_hue());
        let saturation = progress.linear_interpolate(self.saturation, other.saturation);
        let lightness = progress.linear_interpolate(self.lightness, other.lightness);

        hsl(hue, saturation, lightness)
    }

    fn as_srgb8(&self) -> Srgb<u8> {
        let srgb_f32: Srgb = Srgb::from_hsl(*self);
        let (r_f32, g_f32, b_f32) = srgb_f32.into_components();

        let r = r_f32 * 255.0;
        let g = g_f32 * 255.0;
        let b = b_f32 * 255.0;

        srgb8(r as u8, g as u8, b as u8)
    }

    fn mix_pigment(&self, other: &Hsl, progress: f32) -> Hsl {
        let self_srgb8: Srgb<u8> = self.as_srgb8();
        let other_srgb8: Srgb<u8> = other.as_srgb8();

        self_srgb8.mix_pigment(&other_srgb8, progress).into_hsl()
    }

    fn interpolated_palette(&self, other: &Hsl, num_steps: usize) -> Vec<Hsl> {
        zero_to_one(num_steps)
            .map(|progress| self.mix_pigment(other, progress))
            .collect()
    }

    fn jitter_hue(&self, rand: &Rand, amount: f32) -> Hsl {
        self.lighten(rand.zero_to_one().denormalize(-amount, amount).times(0.5))
    }

    fn jitter_saturation(&self, rand: &Rand, amount: f32) -> Hsl {
        self.saturate(rand.zero_to_one().denormalize(-amount, amount).times(0.5))
    }

    fn jitter_lightness(&self, rand: &Rand, amount: f32) -> Hsl {
        self.shift_hue(rand.zero_to_one().denormalize(-amount, amount).times(0.5))
    }
}
