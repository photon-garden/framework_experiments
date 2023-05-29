use geo_booleanop::boolean::BooleanOp;
use std::ops::RangeInclusive;

use super::RectExtension;
use crate::prelude::*;
use itertools::Itertools;

use super::Point2Extension;

pub type Path2 = Vec<Point2>;

type XRange = RangeInclusive<f32>;
type YRange = RangeInclusive<f32>;

pub trait Path2Extension {
    fn from_geo_polygon_exterior(polygon: geo::Polygon<f32>) -> Path2;
    fn straight_path(start: Point2, end: Point2, num_segments: usize) -> Path2;
    fn ellipse(rect: Rect, resolution: usize) -> Path2;
    fn average_first_and_last(&self) -> Point2;
    fn average(&self) -> Point2;
    fn average_y(&self) -> f32;
    fn average_x(&self) -> f32;
    fn regular_polygon<GetRadius>(
        center: &Point2,
        num_points: usize,
        get_radius: GetRadius,
    ) -> Path2
    where
        GetRadius: FnMut(NormalizedF32) -> NormalizedF32;
    fn smooth(&self, tightness: f32, repeats: u8) -> Path2;
    fn smooth_polygon(&self, tightness: f32, repeats: u8) -> Path2;
    fn sum(&self) -> Point2;
    fn jitter(&self, rand: &Rand, x_jitter: f32, y_jitter: f32) -> Path2;
    fn jitter_mut(&mut self, rand: &Rand, x_jitter: f32, y_jitter: f32);
    fn gaussian_jitter(&self, rand: &Rand, x_jitter: f32, y_jitter: f32) -> Path2;
    fn polygon_contains(&self, point: &Point2) -> bool;
    fn denormalize_points(&self, rect: &Rect) -> Vec<Point2>;
    fn width_denormalize_xy(&self, rect: &Rect) -> Vec<Point2>;
    fn height_denormalize_xy(&self, rect: &Rect) -> Vec<Point2>;
    fn perlin_jitter_rotation(&self, rand: &Rand, distance: f32) -> Path2;
    fn as_shell<F>(&self, magnitude_at: F) -> Shell
    where
        F: Fn(f32, &Point2) -> [f32; 2];
    fn split_into_shell(&self) -> Shell;
    fn as_shell_iter<F>(&self, magnitude_at: F) -> ShellIterator<'_, F>
    where
        F: Fn(f32, &Point2) -> [f32; 2];
    fn enblobben(&self, rand: &Rand, frequency: f32, amplitude: f32) -> Shell;
    fn find_closest(&self, target: &Point2) -> &Point2;
    fn find_closest_with_index(&self, target: &Point2) -> (usize, &Point2);
    fn normalize(&self) -> Vec<Point2>;
    fn x_y_range(&self) -> (f32, f32, f32, f32);
    fn invert_normalized_y(&self) -> Path2;
    fn as_geo_polygon(&self) -> geo::Polygon<f32>;
    fn random_point_in_bounding_box(&self, rand: &Rand) -> Point2;
    fn lerp(&self, progress: f32, other: &Path2) -> Path2;
    fn distance_lerpable(self) -> LerpablePath2;
    fn resample(self, resolution: usize) -> Path2;
    fn multiply_resolution(self, multiplier: usize) -> Path2;
    fn dash(&self, dashification: NormalizedF32, dash_size: NormalizedF32) -> Vec<Path2>;
    fn shrink_around_center(self, new_length: NormalizedF32) -> Path2;
    fn times(self, multiplier: f32) -> Path2;
    fn plus_xy(self, point: &Point2) -> Path2;
    fn subtract_from_x(self, amount: f32) -> Path2;
    fn subtract_from_y(self, amount: f32) -> Path2;
    fn intersections(&self, other: &Path2) -> Vec<Path2>;
    fn bounding_box(&self) -> Rect;
    fn walking_length(&self) -> f32;
}

impl Path2Extension for Path2 {
    // geo::Polygons have both an inner and an exterior line string. This just
    // looks at the exterior.
    fn from_geo_polygon_exterior(polygon: geo::Polygon<f32>) -> Path2 {
        polygon
            .exterior()
            .points()
            .map(|point| pt2(point.x(), point.y()))
            .collect()
    }

    fn straight_path(start: Point2, end: Point2, num_segments: usize) -> Path2 {
        zero_to_one(num_segments)
            .map(|progress| start.lerp(end, progress))
            .collect()
    }

    fn ellipse(rect: Rect, resolution: usize) -> Path2 {
        nannou::geom::Ellipse::new(rect, resolution as f32)
            .circumference()
            .into_iter()
            .map(|[x, y]| pt2(x, y))
            .collect_vec()
    }

    fn average_first_and_last(&self) -> Point2 {
        let start = self.first().unwrap();
        let end = self.last().unwrap();

        start.plus_xy(end).divided_by(2.0)
    }

    fn average(&self) -> Point2 {
        self.sum().divided_by(self.len() as f32)
    }

    fn average_x(&self) -> f32 {
        self.iter().map(|point| point.x).average()
    }

    fn average_y(&self) -> f32 {
        self.iter().map(|point| point.y).average()
    }

    fn regular_polygon<GetRadius>(
        center: &Point2,
        num_points: usize,
        mut get_radius: GetRadius,
    ) -> Path2
    where
        GetRadius: FnMut(NormalizedF32) -> NormalizedF32,
    {
        zero_to_one(num_points)
            .map(|angle| {
                let radius = get_radius(angle);
                let radians = angle.turns_to_radians();
                let offset_x = radians.cos().times(radius);
                let offset_y = radians.sin().times(radius);

                pt2(center.x + offset_x, center.y + offset_y)
            })
            .collect()
    }

    // Chaikin's algorithm.
    fn smooth(&self, tightness: f32, repeats: u8) -> Path2 {
        smooth_line_or_polygon_repeatedly(self, tightness, repeats, false)
    }

    fn smooth_polygon(&self, tightness: f32, repeats: u8) -> Path2 {
        smooth_line_or_polygon_repeatedly(self, tightness, repeats, true)
    }

    fn sum(&self) -> Point2 {
        self.iter().fold(Point2::ZERO, |accumulator, point| {
            accumulator.plus_xy(point)
        })
    }

    fn jitter(&self, rand: &Rand, x_jitter: f32, y_jitter: f32) -> Path2 {
        let jittery_path: Path2 = self
            .iter()
            .map(|point| point.jitter(rand, x_jitter, y_jitter))
            .collect();

        jittery_path
    }

    fn jitter_mut(&mut self, rand: &Rand, x_jitter: f32, y_jitter: f32) {
        for point in self.iter_mut() {
            point.jitter_mut(rand, x_jitter, y_jitter);
        }
    }

    fn gaussian_jitter(&self, rand: &Rand, x_jitter: f32, y_jitter: f32) -> Path2 {
        self.iter()
            .map(|point| point.gaussian_jitter(rand, x_jitter, y_jitter))
            .collect()
    }

    fn perlin_jitter_rotation(&self, rand: &Rand, distance: f32) -> Path2 {
        self.iter()
            .map(|point| point.perlin_jitter_rotation(rand, distance))
            .collect()
    }

    fn polygon_contains(&self, point: &Point2) -> bool {
        let polygon = nannou::geom::Polygon::new(self.clone());
        let contains = polygon.contains(point);
        contains.is_some()
    }

    fn denormalize_points(&self, rect: &Rect) -> Vec<Point2> {
        self.iter()
            .map(|point| rect.denormalize_xy(point))
            .collect()
    }

    fn width_denormalize_xy(&self, rect: &Rect) -> Vec<Point2> {
        self.iter()
            .map(|point| rect.width_denormalize_xy(point))
            .collect()
    }

    fn height_denormalize_xy(&self, rect: &Rect) -> Vec<Point2> {
        self.iter()
            .map(|point| rect.height_denormalize_xy(point))
            .collect()
    }

    fn normalize(&self) -> Vec<Point2> {
        let (x_min, x_max, y_min, y_max) = self.x_y_range();

        self.iter()
            .map(|point| point.normalize_in_range(x_min, x_max, y_min, y_max))
            .collect()
    }

    fn x_y_range(&self) -> (f32, f32, f32, f32) {
        let first = self.first().unwrap();

        let mut x_min = first.x;
        let mut y_min = first.y;

        let mut x_max = first.x;
        let mut y_max = first.y;

        for point in self.iter() {
            x_min = point.x.min(x_min);
            y_min = point.y.min(y_min);

            x_max = point.x.max(x_max);
            y_max = point.y.max(y_max);
        }

        (x_min, x_max, y_min, y_max)
    }

    fn as_shell_iter<F>(&self, magnitude_at: F) -> ShellIterator<'_, F>
    where
        F: Fn(f32, &Point2) -> [f32; 2],
    {
        ShellIterator {
            points: self,
            index: 0,
            magnitude_at,
        }
    }

    fn as_shell<F>(&self, magnitude_at: F) -> Shell
    where
        F: Fn(f32, &Point2) -> [f32; 2],
    {
        self.as_shell_iter(magnitude_at).collect()
    }

    fn split_into_shell(&self) -> Shell {
        let halfway_index = self.len() / 2;
        let (first_half, second_half) = self.split_at(halfway_index);

        let first_half_iterator = first_half.iter();

        // If resolution is odd, prepend a copy of the first element to the end of
        // the second half, so that both the first and second halves have the same length.
        let second_half_iterator: Box<dyn Iterator<Item = &Point2>> = if self.len().is_odd() {
            let first_element = std::iter::once(self.first().unwrap());
            second_half.iter().chain(first_element).rev().into_box()
        } else {
            second_half.iter().into_box()
        };

        first_half_iterator
            .zip(second_half_iterator)
            .map(|(left, right)| [*left, *right])
            .collect_vec()
    }

    fn enblobben(&self, rand: &Rand, frequency: f32, amplitude: f32) -> Shell {
        self.as_shell(|_progress, point| {
            let noise = rand.super_simplex_curl(&point.times(frequency));

            let left_magnitude = noise.x * amplitude;
            let right_magnitude = noise.y * amplitude;

            [left_magnitude, right_magnitude]
        })
    }

    fn find_closest(&self, target: &Point2) -> &Point2 {
        let (_index, point) = self.find_closest_with_index(target);
        point
    }

    fn find_closest_with_index(&self, target: &Point2) -> (usize, &Point2) {
        let mut closest_so_far = self.first().unwrap();
        let mut index_of_closest_so_far = 0;
        let mut smallest_distance_so_far = target.distance_squared(*closest_so_far);

        for (index, point) in self.iter().enumerate() {
            let distance = target.distance_squared(*point);

            if distance < smallest_distance_so_far {
                closest_so_far = point;
                index_of_closest_so_far = index;
                smallest_distance_so_far = distance;
            }
        }

        (index_of_closest_so_far, closest_so_far)
    }

    fn invert_normalized_y(&self) -> Path2 {
        self.iter()
            .map(|point| pt2(point.x, 1.0 - point.y))
            .collect()
    }

    fn as_geo_polygon(&self) -> geo::Polygon<f32> {
        let line_string: geo::LineString<f32> = self
            .iter()
            .map(|point| (point.x, point.y))
            .collect::<Vec<(f32, f32)>>()
            .into();

        geo::Polygon::new(line_string, vec![])
    }

    fn random_point_in_bounding_box(&self, rand: &Rand) -> Point2 {
        let first_point = rand.element(self);
        let second_point = rand.element(self);
        let normalized_distance_between = rand.zero_to_one();

        first_point.lerp(normalized_distance_between, second_point)
    }

    fn lerp(&self, progress: f32, other: &Path2) -> Path2 {
        if self.len() != other.len() {
            panic!("Tried to lerp between two paths, but they were different lengths.");
        }

        self.iter()
            .zip(other.iter())
            .map(|(a, b)| a.lerp(progress, b))
            .collect()
    }

    fn distance_lerpable(self) -> LerpablePath2 {
        DistanceLerpablePath::from_path2(self)
    }

    // Resamples self to a path with resolution points,
    // where each segment has the same length.
    fn resample(self, resolution: usize) -> Path2 {
        let lerpable_path = self.distance_lerpable();
        zero_to_one(resolution)
            .map(|progress| lerpable_path.lerp(progress))
            .collect_vec()
    }

    fn multiply_resolution(self, multiplier: usize) -> Path2 {
        let len = self.len();
        self.resample(len * multiplier)
    }

    fn dash(&self, dashification: NormalizedF32, dash_size: NormalizedF32) -> Vec<Path2> {
        let biggest_chunks = (self.len() as f32 / 2.0).ceil();
        let smallest_chunks = 1.0;
        let chunk_size = dashification
            .linear_interpolate(biggest_chunks, smallest_chunks)
            .round() as usize;

        self.chunks(chunk_size)
            .map(|chunk| chunk.to_vec().shrink_around_center(dash_size))
            .collect()
    }

    fn shrink_around_center(self, new_length: NormalizedF32) -> Path2 {
        let normalized_center = 0.5;
        let radius = new_length / 2.0;

        let normalized_start_index = normalized_center.minus(radius).max(0.0).round();
        let start_index = self.denormalize_index(normalized_start_index);

        let normalized_end_index = normalized_center.plus(radius).min(1.0);
        let end_index = self.denormalize_index(normalized_end_index);

        self[start_index..=end_index].to_vec()
    }

    fn times(self, multiplier: f32) -> Path2 {
        self.into_iter()
            .map(|point| point.times(multiplier))
            .collect()
    }

    fn plus_xy(self, addition: &Point2) -> Path2 {
        self.into_iter()
            .map(|point| point.plus_xy(addition))
            .collect()
    }

    fn subtract_from_x(self, amount: f32) -> Path2 {
        self.into_iter()
            .map(|point| point.subtract_from_x(amount))
            .collect()
    }

    fn subtract_from_y(self, amount: f32) -> Path2 {
        self.into_iter()
            .map(|point| point.subtract_from_y(amount))
            .collect()
    }

    fn intersections(&self, other: &Path2) -> Vec<Path2> {
        let self_polygon = self.as_geo_polygon();
        let other_polygon = other.as_geo_polygon();

        // For some reason this panics sometimes. If that happens just assume there are
        // no intersections.
        std::panic::catch_unwind(|| self_polygon.intersection(&other_polygon))
            .map(|intersection_multi_polygon| {
                intersection_multi_polygon
                    .into_iter()
                    .map(Path2::from_geo_polygon_exterior)
                    .collect()
            })
            .unwrap_or_else(|_| vec![])
    }

    fn bounding_box(&self) -> Rect {
        let mut min_x = f32::MAX;
        let mut min_y = f32::MAX;

        let mut max_x = f32::MIN;
        let mut max_y = f32::MIN;

        for point in self.iter() {
            min_x = min_x.min(point.x);
            min_y = min_y.min(point.y);
            max_x = max_x.max(point.x);
            max_y = max_y.max(point.y);
        }

        let bottom_left = pt2(min_x, min_y);
        let top_right = pt2(max_x, max_y);

        Rect::from_corners(bottom_left, top_right)
    }

    // This function doesn't give you the number of elements in the path, it gives you how long
    // the path would be if you had to walk it.
    fn walking_length(&self) -> f32 {
        if self.len() <= 1 {
            return 0.0;
        }

        let mut total_distance_between_points = 0.0;

        for window in self.windows(2) {
            let current_point = &window[0];
            let next_point = &window[1];
            let distance = current_point.distance(*next_point);
            total_distance_between_points += distance;
        }

        total_distance_between_points
    }
}

fn smooth_line_or_polygon_repeatedly(
    path: &Path2,
    tightness: f32,
    repeats: u8,
    is_polygon: bool,
) -> Path2 {
    if repeats == 0 {
        return path.to_owned();
    }

    let mut smoothed = smooth_line_or_polygon(path, tightness, is_polygon);

    for _ in 0..(repeats - 1) {
        smoothed = smooth_line_or_polygon(&smoothed, tightness, is_polygon);
    }

    smoothed
}

fn smooth_line_or_polygon(path: &Path2, tightness: f32, is_polygon: bool) -> Path2 {
    if path.len() < 3 {
        return path.clone();
    }

    let num_line_segments = path.len() - 1;

    // Each line segment creates two points, then we add on the
    // first and last point from the original vector.
    let mut smoothed: Path2 = Vec::with_capacity(num_line_segments * 2 + 2);

    if !is_polygon {
        smoothed.push(*path.first().unwrap());
    }

    let inverse_tightness = 1.0 - tightness;

    for index in 0..path.last_index() {
        let point = path[index];
        let next_point = path[index + 1];

        let line_segment = [point, next_point];

        let near = line_segment.interpolate(tightness);
        let far = line_segment.interpolate(inverse_tightness);

        smoothed.push(near);
        smoothed.push(far);
    }

    if !is_polygon {
        smoothed.push(*path.last().unwrap());
    }

    if is_polygon {
        let first = *path.first().unwrap();
        let mut last = *path.last().unwrap();

        if first == last {
            let next_to_last_index = path.last_index() - 1;
            last = *path.get(next_to_last_index).unwrap();
        }

        let point = last;
        let next_point = first;
        let line_segment = [point, next_point];

        let near = line_segment.interpolate(tightness);
        let far = line_segment.interpolate(inverse_tightness);

        smoothed.push(near);
        smoothed.push(far);
    }

    smoothed
}

pub struct ShellIterator<'a, F>
where
    F: Fn(f32, &Point2) -> [f32; 2],
{
    points: &'a Path2,
    magnitude_at: F,
    index: usize,
}

impl<'a, F> Iterator for ShellIterator<'a, F>
where
    F: Fn(f32, &Point2) -> [f32; 2],
{
    type Item = [Point2; 2];

    fn next(&mut self) -> Option<Self::Item> {
        let len = self.points.len();

        if len < 2 {
            panic!("A Vec needs at least two elements to be turned into a shell.");
        }

        let points = self.points;
        let last_index = self.points.last_index();
        let magnitude_at = &self.magnitude_at;

        loop {
            let current_index = self.index;

            if current_index > last_index {
                return None;
            }

            self.index += 1;

            let progress = current_index as f32 / last_index as f32;
            let window_start;
            let window_end;
            let point_to_translate_from;

            if current_index == last_index {
                window_start = points.get(current_index - 1).unwrap();
                window_end = points.get(current_index).unwrap();
                point_to_translate_from = window_end;
            } else {
                window_start = points.get(current_index).unwrap();
                window_end = points.get(current_index + 1).unwrap();
                point_to_translate_from = window_start;
            }

            let forward = Vec2::between(window_start, window_end).normalize();

            // If window_start and window_end are the same point, the x and y
            // components of forward will be NaN. If that's the case, we skip
            // this iteration of the loop.
            if forward.x.is_nan() {
                continue;
            }

            let [left_magnitude, right_magnitude] = magnitude_at(progress, point_to_translate_from);

            let left_direction = forward.perpendicular_counterclockwise() * left_magnitude;
            let right_direction = forward.perpendicular_clockwise() * right_magnitude;

            let left_point = *point_to_translate_from + left_direction;
            let right_point = *point_to_translate_from + right_direction;

            let pair = [left_point, right_point];

            return Some(pair);
        }
    }
}
