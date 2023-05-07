use itertools::Itertools;

use crate::prelude::*;

pub struct Tube {}

pub type Ring = Path3;
pub type ProgressAlongTube = NormalizedF32;
pub type ProgressAroundRing = NormalizedF32;

impl Tube {
    pub fn rings<GetRingRadius>(
        path: Path3,
        ring_resolution: usize,
        get_ring_radius: GetRingRadius,
    ) -> Vec<Ring>
    where
        GetRingRadius: Fn(ProgressAlongTube, ProgressAroundRing) -> NormalizedF32,
    {
        let mut rings = Vec::with_capacity(path.len());

        for (progress_along_tube, window) in path.windows(2).enumerate_normalized() {
            let current_point = window[0];
            let next_point = window[1];

            let maybe_between = current_point.between(next_point).try_normalize();

            let between = match maybe_between {
                Some(between) => between,
                None => continue,
            };

            let ring = Circle3::points(
                current_point,
                between,
                ring_resolution,
                |progress_around_ring| get_ring_radius(progress_along_tube, progress_around_ring),
            );

            rings.push(ring);
        }

        rings
    }
}
