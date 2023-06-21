use crate::prelude::*;

pub type LineSegment = [Point2; 2];

pub trait LineSegmentExtension {
    fn length2(&self) -> f32;
    fn interpolate(&self, progress: NormalizedF32) -> Point2;
}

impl LineSegmentExtension for LineSegment {
    fn length2(&self) -> f32 {
        self[0].distance_squared(self[1])
    }
    fn interpolate(&self, progress: NormalizedF32) -> Point2 {
        let inverse_progress = 1.0 - progress;
        self[0] * inverse_progress + self[1] * progress
    }
}
