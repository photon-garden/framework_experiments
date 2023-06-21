use crate::prelude::*;
use crate::snapshot::rand::Rand;
use nannou::math::ConvertAngle;
use nannou::prelude::*;
use std::ops::RangeBounds;
use std::ops::RangeInclusive;

pub type NormalizedPoint2 = Point2;
pub type DenormalizedPoint2 = Point2;

pub trait Point2Extension {
    fn reflect(&self, around: &Point2) -> Point2;
    fn lerp(&self, progress: f32, other: &Point2) -> Point2;
    fn jitter(&self, rand: &Rand, x_jitter: f32, y_jitter: f32) -> Point2;
    fn jitter_mut(&mut self, rand: &Rand, x_jitter: f32, y_jitter: f32);
    fn gaussian_jitter(&self, rand: &Rand, x_jitter: f32, y_jitter: f32) -> Point2;
    fn perlin_jitter_rotation(&self, rand: &Rand, distance: f32) -> Point2;
    fn denormalize(&self, rect: &Rect) -> Point2;
    fn denormalize_to_range(&self, min: f32, max: f32) -> Point2;
    fn normalize_in_rect(&self, rect: &Rect) -> Point2;
    fn normalize_in_range(&self, x_min: f32, x_max: f32, y_min: f32, y_max: f32) -> Point2;
    fn outside_normalized_range(&self) -> bool;
    fn inside_normalized_range(&self) -> bool;
    fn loop_normalize(&self, min: &Point2, max: &Point2) -> Point2;
    fn plus_xy(&self, other: &Point2) -> Point2;
    fn plus(&self, amount: f32) -> Point2;
    fn plus_x(&self, amount: f32) -> Point2;
    fn plus_y(&self, amount: f32) -> Point2;
    fn times(&self, multiple: f32) -> Point2;
    fn vector_towards(&self, other: &Point2) -> Vec2;
    fn distance_to_line(&self, line_start: &Point2, line_end: &Point2) -> f32;
    fn average_x_and_y(&self) -> f32;
    fn map(&self, mapper: impl FnOnce(f32, f32) -> Point2) -> Point2;
    fn map_x(&self, mapper: impl FnOnce(f32) -> f32) -> Point2;
    fn map_y(&self, mapper: impl FnOnce(f32) -> f32) -> Point2;
    fn multiply_x_by(&self, multiple: f32) -> Point2;
    fn multiply_y_by(&self, multiple: f32) -> Point2;
    fn subtract_from_x(&self, amount: f32) -> Point2;
    fn subtract_from_y(&self, amount: f32) -> Point2;
    fn floor_to_usize_point2(&self) -> UsizePoint2;
    fn within<Range: RangeBounds<f32> + Clone>(&self, range: Range) -> bool;
    fn normalized_point_to_polar_coordinates(&self) -> [f32; 2];
    fn account_for_window_distortion(&self) -> Point2;
    fn account_for_window_distortion_by_normalizing(&self) -> Point2;
    fn quantize_x(&self, resolution: usize) -> Point2;
    fn quantize_y(&self, resolution: usize) -> Point2;
    fn quantize(&self, resolution: usize) -> Point2;
    fn clamp_normalized(&self) -> Point2;
    fn walk(&self, angle: NumberOfTurns, distance: f32) -> Point2;
}

impl Point2Extension for Point2 {
    fn reflect(&self, around: &Point2) -> Point2 {
        let x_distance = self.x - around.x;
        let y_distance = self.y - around.y;

        let new_x = around.x - x_distance;
        let new_y = around.y - y_distance;

        pt2(new_x, new_y)
    }

    fn lerp(&self, progress: f32, other: &Point2) -> Point2 {
        let inverse_progress = 1.0 - progress;
        *self * inverse_progress + *other * progress
    }

    fn jitter(&self, rand: &Rand, x_jitter: f32, y_jitter: f32) -> Point2 {
        let x_jitter = rand.zero_to_one().denormalize_symmetrically(x_jitter);
        let y_jitter = rand.zero_to_one().denormalize_symmetrically(y_jitter);

        pt2(self.x + x_jitter, self.y + y_jitter)
    }

    fn jitter_mut(&mut self, rand: &Rand, x_jitter: f32, y_jitter: f32) {
        let x_jitter = rand.zero_to_one() * x_jitter;
        let y_jitter = rand.zero_to_one() * y_jitter;

        self.x += x_jitter;
        self.y += y_jitter;
    }

    fn gaussian_jitter(&self, rand: &Rand, x_jitter: f32, y_jitter: f32) -> Point2 {
        let x_jitter = rand.standard_gaussian() * x_jitter;
        let y_jitter = rand.standard_gaussian() * y_jitter;

        pt2(self.x + x_jitter, self.y + y_jitter)
    }

    fn perlin_jitter_rotation(&self, rand: &Rand, distance: f32) -> Point2 {
        let rotation = rand.perlin_xy(self);
        let offset = Vec2::from_angle(rotation.turns_to_radians()) * distance;

        *self + offset
    }

    fn denormalize(&self, rect: &Rect) -> Point2 {
        rect.denormalize_xy(self)
    }

    fn denormalize_to_range(&self, min: f32, max: f32) -> Point2 {
        let new_x = self.x.denormalize(min, max);
        let new_y = self.y.denormalize(min, max);

        pt2(new_x, new_y)
    }

    fn outside_normalized_range(&self) -> bool {
        let normalized_range: RangeInclusive<f32> = 0.0..=1.0;

        !normalized_range.contains(&self.x) || !normalized_range.contains(&self.y)
    }

    fn inside_normalized_range(&self) -> bool {
        let normalized_range: RangeInclusive<f32> = 0.0..=1.0;
        normalized_range.contains(&self.x) && normalized_range.contains(&self.y)
    }

    fn normalize_in_rect(&self, rect: &Rect) -> Point2 {
        let normalized_x = self.x.normalize(rect.left(), rect.right());
        let normalized_y = self.y.normalize(rect.bottom(), rect.top());

        pt2(normalized_x, normalized_y)
    }

    fn normalize_in_range(&self, x_min: f32, x_max: f32, y_min: f32, y_max: f32) -> Point2 {
        let normalized_x = self.x.normalize(x_min, x_max);
        let normalized_y = self.y.normalize(y_min, y_max);

        pt2(normalized_x, normalized_y)
    }

    // Takes a point that goes from (0, 0) to (x_range, y_range)
    // and normalizes it. It handles out of bounds values by looping
    // them back into the normalized range. In contrast, regular
    // normalization just lets the value exceed the normalized range.
    fn loop_normalize(&self, min: &Point2, max: &Point2) -> Point2 {
        pt2(
            self.x.loop_normalize(min.x, max.x),
            self.y.loop_normalize(min.y, max.y),
        )
    }

    fn plus(&self, amount: f32) -> Point2 {
        pt2(self.x + amount, self.y + amount)
    }

    fn plus_x(&self, amount: f32) -> Point2 {
        pt2(self.x + amount, self.y)
    }

    fn plus_y(&self, amount: f32) -> Point2 {
        pt2(self.x, self.y + amount)
    }

    fn plus_xy(&self, other: &Point2) -> Point2 {
        *self + *other
    }

    fn times(&self, multiple: f32) -> Point2 {
        *self * multiple
    }

    fn vector_towards(&self, other: &Point2) -> Vec2 {
        *other - *self
    }

    fn distance_to_line(&self, line_start: &Point2, line_end: &Point2) -> f32 {
        // Formula from https://en.wikipedia.org/wiki/Distance_from_a_point_to_a_line
        let x0 = self.x;
        let y0 = self.y;

        let x1 = line_start.x;
        let y1 = line_start.y;

        let x2 = line_end.x;
        let y2 = line_end.y;

        // Line start and end are the same -- just calculate distance between two points.
        if x1.close_to(x2) && y1.close_to(y2) {
            return self.distance(*line_start);
        }

        let numerator = {
            let lhs = (x2 - x1) * (y1 - y0);
            let rhs = (x1 - x0) * (y2 - y1);
            (lhs - rhs).abs()
        };

        let denominator = {
            let lhs = (x2 - x1).pow(2.0);
            let rhs = (y2 - y1).pow(2.0);
            (lhs + rhs).sqrt()
        };

        numerator / denominator
    }

    fn average_x_and_y(&self) -> f32 {
        (self.x + self.y) / 2.0
    }

    fn map(&self, mapper: impl FnOnce(f32, f32) -> Point2) -> Point2 {
        mapper(self.x, self.y)
    }

    fn map_x(&self, mapper: impl FnOnce(f32) -> f32) -> Point2 {
        self.map(|x, y| {
            let new_x = mapper(x);
            pt2(new_x, y)
        })
    }

    fn map_y(&self, mapper: impl FnOnce(f32) -> f32) -> Point2 {
        self.map(|x, y| {
            let new_y = mapper(y);
            pt2(x, new_y)
        })
    }

    fn multiply_x_by(&self, multiple: f32) -> Point2 {
        pt2(self.x * multiple, self.y)
    }

    fn multiply_y_by(&self, multiple: f32) -> Point2 {
        pt2(self.x, self.y * multiple)
    }

    fn subtract_from_x(&self, amount: f32) -> Point2 {
        pt2(self.x - amount, self.y)
    }

    fn subtract_from_y(&self, amount: f32) -> Point2 {
        pt2(self.x, self.y - amount)
    }

    fn floor_to_usize_point2(&self) -> UsizePoint2 {
        let x = self.x.floor() as usize;
        let y = self.y.floor() as usize;
        usize_pt2(x, y)
    }

    fn within<Range: RangeBounds<f32> + Clone>(&self, range: Range) -> bool {
        self.x.within(range.clone()) && self.y.within(range)
    }

    fn normalized_point_to_polar_coordinates(&self) -> [f32; 2] {
        let denormalized = self.denormalize_to_range(-1.0, 1.0);
        let radius = denormalized.distance(pt2(0.0, 0.0)) * 0.5;
        let angle = denormalized.y.atan2(denormalized.x).rad_to_turns();
        [radius, angle]
    }

    fn account_for_window_distortion(&self) -> Point2 {
        pt2(self.x * aspect_ratio(), self.y)
        // *self
    }

    fn account_for_window_distortion_by_normalizing(&self) -> Point2 {
        // Aspect ratio is width / height. So when correcting for
        // window distortion, we can assume y radius is 0.5 and
        // x radius is half the aspect ratio.
        let x_radius = aspect_ratio() / 2.0;
        let _y_radius = 0.5;

        let x = self.x.normalize(0.5 - x_radius, 0.5 + x_radius);
        let y = self.y;

        pt2(x, y)
    }

    fn quantize_x(&self, resolution: usize) -> Point2 {
        pt2(self.x.quantize(resolution), self.y)
    }

    fn quantize_y(&self, resolution: usize) -> Point2 {
        pt2(self.x, self.y.quantize(resolution))
    }

    fn quantize(&self, resolution: usize) -> Point2 {
        pt2(self.x.quantize(resolution), self.y.quantize(resolution))
    }

    fn clamp_normalized(&self) -> Point2 {
        let zero = pt2(0.0, 0.0);
        let one = pt2(1.0, 1.0);
        self.clamp(zero, one)
    }

    fn walk(&self, turns: NumberOfTurns, distance: f32) -> Point2 {
        let movement = Vec2::from_angle(turns).times(distance);
        *self + movement
    }
}

impl From<UsizePoint2> for Point2 {
    fn from(other: UsizePoint2) -> Self {
        pt2(other.x as f32, other.y as f32)
    }
}

impl From<&UsizePoint2> for Point2 {
    fn from(other: &UsizePoint2) -> Self {
        pt2(other.x as f32, other.y as f32)
    }
}
