use super::path2::*;
use crate::snapshot::rand::Rand;

pub type Paths = Vec<Path2>;

pub trait PathsExtension {
    fn smooth(&self, tightness: f32, repeats: u8) -> Paths;
    fn jitter(&self, rand: &Rand, x_jitter: f32, y_jitter: f32) -> Paths;
    fn gaussian_jitter(&self, rand: &Rand, x_jitter: f32, y_jitter: f32) -> Paths;
}

impl PathsExtension for Paths {
    fn smooth(&self, tightness: f32, repeats: u8) -> Paths {
        self.iter()
            .map(|path| path.smooth(tightness, repeats))
            .collect()
    }
    fn jitter(&self, rand: &Rand, x_jitter: f32, y_jitter: f32) -> Paths {
        self.iter()
            .map(|path| path.jitter(rand, x_jitter, y_jitter))
            .collect()
    }
    fn gaussian_jitter(&self, rand: &Rand, x_jitter: f32, y_jitter: f32) -> Paths {
        self.iter()
            .map(|path| path.gaussian_jitter(rand, x_jitter, y_jitter))
            .collect()
    }
}
