use crate::prelude::*;
use itertools::Itertools;
use nannou::geom::{Quad, Tri};

pub type Shell3 = Vec<[Point3; 2]>;

pub trait Shell3Extension {
    fn into_path(self) -> Path3;
    fn sides(self) -> [Path3; 2];
    fn triangles(&self) -> Vec<Tri>;
    fn interior_point(
        &self,
        distance_along: NormalizedF32,
        distance_between_halves: NormalizedF32,
    ) -> Point3;
    fn points(&self) -> Vec<Point3>;
}

impl Shell3Extension for Shell3 {
    fn into_path(self) -> Path3 {
        if self.is_empty() {
            return vec![];
        }

        let [mut left, mut right] = self.sides();

        right.reverse();
        left.append(&mut right);

        // Close the loop.
        let first = *left.first().unwrap();
        left.push(first);

        left
    }

    fn sides(self) -> [Path3; 2] {
        let side_len = self.len() / 2;
        let mut left = Vec::with_capacity(side_len);
        let mut right = Vec::with_capacity(side_len);

        for [left_point, right_point] in self {
            left.push(left_point);
            right.push(right_point);
        }

        [left, right]
    }

    fn interior_point(
        &self,
        distance_along: NormalizedF32,
        distance_between_halves: NormalizedF32,
    ) -> Point3 {
        let [left, right] = self.normalized_get(distance_along);
        left.lerp(*right, distance_between_halves)
    }

    fn triangles(&self) -> Vec<Tri> {
        let resolution = self.len();

        let mut triangles = vec![];

        for pair_window in self.windows(2) {
            let [current_left, current_right] = pair_window[0];
            let [next_left, next_right] = pair_window[1];

            let progress_iterator = zero_to_one(resolution).collect_vec();
            let progress_windows = progress_iterator.windows(2);

            for progress_window in progress_windows {
                let current_progress = progress_window[0];
                let next_progress = progress_window[1];

                let a = current_left.lerp(current_right, current_progress);
                let b = current_left.lerp(current_right, next_progress);
                let c = next_left.lerp(next_right, next_progress);
                let d = next_left.lerp(next_right, current_progress);

                let quad = Quad([a, b, c, d]);
                for triangle in quad.triangles_iter() {
                    triangles.push(triangle);
                }
            }
        }

        triangles
    }

    fn points(&self) -> Vec<Point3> {
        let resolution = self.len();

        self.iter()
            .flat_map(|[left, right]| left.multi_lerp(*right, resolution))
            .collect()
    }
}
