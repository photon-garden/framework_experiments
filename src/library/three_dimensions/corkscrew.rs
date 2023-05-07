use itertools::Itertools;

use crate::prelude::*;

pub struct Corkscrew {}

impl Corkscrew {
    pub fn path(path: Path3, radius: NormalizedF32, num_turns: f32, phase: NumberOfTurns) -> Path3 {
        path.windows(2)
            .enumerate_normalized()
            .filter_map(|(progress, window)| {
                let current_point = window[0];
                let next_point = window[1];
                current_point
                    .between(next_point)
                    .try_normalize()
                    .map(|between| {
                        let turns = progress.denormalize(0.0, num_turns) + phase;
                        Circle3::point(current_point, between, radius, turns)
                    })
            })
            .collect_vec()
    }
}
