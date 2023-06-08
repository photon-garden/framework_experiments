use crate::prelude::*;
use nannou::color::Hue;
use nannou::color::Saturate;
use nannou::color::Shade;

pub trait HslaExtension {
    fn lerp(&self, other: &Hsla, progress: f32) -> Hsla;
    fn normalized_hue(&self) -> NormalizedF32;
    fn jitter_hue(&self, rand: &Rand, amount: f32) -> Hsla;
    fn jitter_saturation(&self, rand: &Rand, amount: f32) -> Hsla;
    fn jitter_lightness(&self, rand: &Rand, amount: f32) -> Hsla;
}

impl HslaExtension for Hsla {
    fn normalized_hue(&self) -> NormalizedF32 {
        self.hue.to_normalized()
    }

    fn lerp(&self, other: &Hsla, progress: f32) -> Hsla {
        let hue = progress.linear_interpolate(self.normalized_hue(), other.normalized_hue());
        let saturation = progress.linear_interpolate(self.saturation, other.saturation);
        let lightness = progress.linear_interpolate(self.lightness, other.lightness);
        let alpha = progress.linear_interpolate(self.alpha, other.alpha);

        hsla(hue, saturation, lightness, alpha)
    }

    fn jitter_hue(&self, rand: &Rand, amount: f32) -> Hsla {
        self.lighten(rand.zero_to_one().denormalize(-amount, amount).times(0.5))
    }

    fn jitter_saturation(&self, rand: &Rand, amount: f32) -> Hsla {
        self.saturate(rand.zero_to_one().denormalize(-amount, amount).times(0.5))
    }

    fn jitter_lightness(&self, rand: &Rand, amount: f32) -> Hsla {
        self.shift_hue(rand.zero_to_one().denormalize(-amount, amount).times(0.5))
    }
}
