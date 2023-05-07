pub use crate::prelude::*;

pub type Shell = Vec<[Point2; 2]>;

pub trait ShellExtension {
    fn from_sides(sides: [Path2; 2]) -> Shell;
    fn normalized_circle(num_segments: usize, radius: f32, center: Point2) -> Shell;
    fn fill_with_points<NumPointsAt, PointBetween>(
        &self,
        num_points_at: NumPointsAt,
        point_between: PointBetween,
    ) -> Vec<Point2>
    where
        NumPointsAt: Fn(f32) -> usize,
        PointBetween: Fn(PointBetweenParams) -> Point2;
    fn lerp_sides(&self, progress: f32) -> Path2;
    fn sides(self) -> [Path2; 2];
    fn into_path(self) -> Path2;
    fn interior_point(
        &self,
        distance_along: NormalizedF32,
        distance_between_halves: NormalizedF32,
    ) -> Point2;
    fn interior_point_xy(&self, xy: &Point2) -> Point2;
    fn interior_point_with_index(
        &self,
        distance_along: NormalizedF32,
        distance_between_halves: NormalizedF32,
    ) -> (usize, Point2);
    fn map_shell_segments(&self, mapper: impl Fn(&Point2) -> Point2) -> Shell;
    fn resample(self, resolution: usize) -> Shell;
    fn smooth(self, tightness: f32, repeats: u8) -> Shell;
    fn map_sides(self, mapper: impl Fn(Path2) -> Path2) -> Shell;
}

pub struct PointBetweenParams<'a> {
    pub progress_along_line: f32,
    pub normalized_index: f32,
    pub left_point: &'a Point2,
    pub right_point: &'a Point2,
}

impl ShellExtension for Shell {
    fn from_sides(sides: [Path2; 2]) -> Shell {
        let [left, right] = sides;
        left.into_iter()
            .zip(right.into_iter())
            .map(|(left, right)| [left, right])
            .collect()
    }

    fn normalized_circle(num_segments: usize, radius: f32, center: Point2) -> Shell {
        let circle = Circle {
            center_x: center.x,
            center_y: center.y,
            radius,
        };

        let spine_start = circle.left();
        let spine_end = circle.right();
        let spine = Path2::straight_path(spine_start, spine_end, num_segments);

        spine.as_shell(|_progress, point| {
            let radius = circle.vertical_distance_to_edge(point.x);
            [radius, radius]
        })
    }

    fn fill_with_points<NumPointsAt, PointBetween>(
        &self,
        num_points_at: NumPointsAt,
        point_between: PointBetween,
    ) -> Vec<Point2>
    where
        NumPointsAt: Fn(f32) -> usize,
        PointBetween: Fn(PointBetweenParams) -> Point2,
    {
        let len = self.len();
        self.iter()
            .enumerate()
            .flat_map(|(index, current_pair)| {
                let [left_point, right_point] = current_pair;

                let progress = index as f32 / len as f32;
                let num_points_between_current_pair = num_points_at(progress);
                let mut points_between_current_pair =
                    Vec::with_capacity(num_points_between_current_pair);

                for index in 0..num_points_between_current_pair {
                    let normalized_index = index as f32 / num_points_between_current_pair as f32;
                    let params = PointBetweenParams {
                        progress_along_line: progress,
                        normalized_index,
                        left_point,
                        right_point,
                    };
                    let point = point_between(params);
                    points_between_current_pair.push(point);
                }

                points_between_current_pair
            })
            .collect()
    }
    fn lerp_sides(&self, progress: f32) -> Path2 {
        self.iter()
            .map(|[left_point, right_point]| left_point.lerp(progress, right_point))
            .collect()
    }
    fn sides(self) -> [Path2; 2] {
        let side_len = self.len() / 2;
        let mut left = Vec::with_capacity(side_len);
        let mut right = Vec::with_capacity(side_len);

        for [left_point, right_point] in self {
            left.push(left_point);
            right.push(right_point);
        }

        [left, right]
    }
    fn into_path(self) -> Path2 {
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

    fn interior_point(
        &self,
        distance_along: NormalizedF32,
        distance_between_halves: NormalizedF32,
    ) -> Point2 {
        let [left, right] = self.normalized_get(distance_along);
        left.lerp(distance_between_halves, right)
    }

    fn interior_point_xy(&self, xy: &Point2) -> Point2 {
        let distance_along = xy.y;
        let distance_between = xy.x;
        self.interior_point(distance_along, distance_between)
    }

    fn interior_point_with_index(
        &self,
        distance_along: NormalizedF32,
        distance_between_halves: NormalizedF32,
    ) -> (usize, Point2) {
        let (index, pair) = self.normalized_get_with_index(distance_along);
        let [left, right] = pair;
        let point = left.lerp(distance_between_halves, right);
        (index, point)
    }

    fn map_shell_segments(&self, mapper: impl Fn(&Point2) -> Point2) -> Shell {
        self.iter().map(|[a, b]| [mapper(a), mapper(b)]).collect()
    }

    fn resample(self, resolution: usize) -> Shell {
        self.map_sides(|side| side.resample(resolution))
    }

    fn smooth(self, tightness: f32, repeats: u8) -> Shell {
        self.map_sides(|side| side.smooth(tightness, repeats))
    }

    fn map_sides(self, mapper: impl Fn(Path2) -> Path2) -> Shell {
        let mapped_sides = self.sides().map(mapper);
        Shell::from_sides(mapped_sides)
    }
}
