use itertools::Itertools;

use crate::prelude::*;

pub type Path3 = Vec<Point3>;

pub trait Path3Extension {
    fn straight_path(start: Point3, end: Point3, num_segments: usize) -> Path3;
    fn denormalize_x_y(&self, rect: &Rect) -> Vec<Point2>;
    fn to_path2(&self) -> Vec<Point2>;
    fn polygon_contains(&self, point: &Point3) -> bool;
    fn find_closest(&self, point: &Point3) -> &Point3;
    fn smooth(&self, tightness: f32, repeats: u8) -> Path3;
    fn smooth_once(&self, tightness: f32) -> Path3;
    fn resample(self, resolution: usize) -> Path3;
    fn multiply_resolution(self, multiplier: usize) -> Path3;
    fn normalized_betweens(self) -> Path3;
}

impl Path3Extension for Path3 {
    fn straight_path(start: Point3, end: Point3, num_segments: usize) -> Path3 {
        zero_to_one(num_segments)
            .map(|progress| start.lerp(end, progress))
            .collect()
    }

    fn denormalize_x_y(&self, rect: &Rect) -> Vec<Point2> {
        self.iter()
            .map(|landmark| rect.denormalize_x_y(landmark.x, landmark.y))
            .collect()
    }
    fn to_path2(&self) -> Vec<Point2> {
        self.iter().map(|point| pt2(point.x, point.y)).collect()
    }
    fn polygon_contains(&self, point: &Point3) -> bool {
        let polygon = nannou::geom::Polygon::new(self.clone());
        let contains = polygon.contains(point);
        contains.is_some()
    }
    fn find_closest(&self, target: &Point3) -> &Point3 {
        let mut closest_so_far = self.first().unwrap();
        let mut smallest_distance_so_far = target.distance_squared(*closest_so_far);

        for point in self.iter() {
            let distance = target.distance_squared(*point);

            if distance < smallest_distance_so_far {
                closest_so_far = point;
                smallest_distance_so_far = distance;
            }
        }

        closest_so_far
    }

    fn smooth(&self, tightness: f32, repeats: u8) -> Path3 {
        if repeats == 0 {
            return self.to_owned();
        }

        let mut smoothed = self.smooth_once(tightness);

        for _ in 0..(repeats - 1) {
            smoothed = smoothed.smooth_once(tightness);
        }

        smoothed
    }

    fn smooth_once(&self, tightness: f32) -> Path3 {
        let num_line_segments = self.len() - 1;

        // Each line segment creates two points, then we add on the
        // first and last point from the original vector.
        let mut smoothed: Path3 = Vec::with_capacity(num_line_segments * 2 + 2);

        // if !is_polygon {
        smoothed.push(*self.first().unwrap());
        // }

        let inverse_tightness = 1.0 - tightness;

        for index in 0..self.last_index() {
            let point = self[index];
            let next_point = self[index + 1];

            let near = point.lerp(next_point, tightness);
            let far = point.lerp(next_point, inverse_tightness);

            smoothed.push(near);
            smoothed.push(far);
        }

        // if !is_polygon {
        smoothed.push(*self.last().unwrap());
        // }

        // if is_polygon {
        //     let first = *path.first().unwrap();
        //     let mut last = *path.last().unwrap();

        //     if first == last {
        //         let next_to_last_index = path.last_index() - 1;
        //         last = *path.get(next_to_last_index).unwrap();
        //     }

        //     let point = last;
        //     let next_point = first;
        //     let line_segment = [point, next_point];

        //     let near = line_segment.interpolate(tightness);
        //     let far = line_segment.interpolate(inverse_tightness);

        //     smoothed.push(near);
        //     smoothed.push(far);
        // }

        smoothed
    }

    fn multiply_resolution(self, multiplier: usize) -> Path3 {
        let len = self.len();
        self.resample(len * multiplier)
    }

    // Resamples self to a path with resolution points,
    // where each segment has the same length.
    fn resample(self, resolution: usize) -> Path3 {
        let lerpable_path = DistanceLerpablePath::from_path3(self);
        zero_to_one(resolution)
            .map(|progress| lerpable_path.lerp(progress))
            .collect_vec()
    }

    // If you have:
    //   let path3 = vec![a, b, c, d];
    // This method returns:
    //   vec![
    //     normalized_vec_between_a_and_b,
    //     normalized_vec_between_b_and_c,
    //     normaliezd_vec_between_c_and_d
    //   ]
    // and so on.
    fn normalized_betweens(self) -> Vec<Vec3> {
        self.windows(2)
            .filter_map(|window| {
                let current_point = window[0];
                let next_point = window[1];
                current_point.between(next_point).try_normalize()
            })
            .collect()
    }
}
