use nannou::color::RgbHue;

pub trait RgbHueExtension {
    fn to_normalized(&self) -> f32;
}

impl RgbHueExtension for RgbHue {
    fn to_normalized(&self) -> f32 {
        self.to_positive_degrees() / 360.0
    }
}
