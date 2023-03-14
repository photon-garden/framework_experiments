use std::ops::Range;
use std::ops::RangeBounds;
use std::ops::RangeInclusive;

use crate::prelude::*;

pub type NormalizedF32 = f32;

pub trait F32Extension {
    fn rescale(&self, input_min: f32, input_max: f32, output_min: f32, output_max: f32) -> f32;
    fn is_between(&self, min: f32, max: f32) -> bool;
    fn is_normalized(&self) -> bool;
    fn normalize(&self, input_min: f32, input_max: f32) -> f32;
    fn denormalize(&self, output_min: f32, output_max: f32) -> f32;
    fn denormalize_to_range(&self, range: &RangeInclusive<f32>) -> f32;
    fn denormalize_symmetrically(&self, radius: f32) -> f32;
    fn lerp_w(&self, rect: &Rect) -> f32;
    fn lerp_h(&self, rect: &Rect) -> f32;
    fn times(&self, other: f32) -> f32;
    fn plus(&self, other: f32) -> f32;
    fn minus(&self, other: f32) -> f32;
    fn divided_by(&self, other: f32) -> f32;
    fn clamp(&self, min: f32, max: f32) -> f32;
    fn normalize_w(&self, rect: &Rect) -> f32;
    fn normalize_h(&self, rect: &Rect) -> f32;
    fn turns_to_radians(&self) -> f32;
    fn jitter(&self, rand: &Rand, amplitude: f32) -> f32;
    fn normalized_cos(&self) -> f32;
    fn normalized_sin(&self) -> f32;
    fn is_integer(&self) -> bool;
    fn loop_normalize(&self, min: f32, max: f32) -> f32;
    fn looped(&self) -> f32;
    fn close_to(&self, other: f32) -> bool;
    fn far_from(&self, other: f32) -> bool;
    fn linear_interpolate(&self, start: f32, end: f32) -> f32;
    fn oscillate_triangle(&self) -> f32;
    fn flattened_triangle(&self, max: NormalizedF32) -> NormalizedF32;
    fn oscillate_eased<EasingFunction>(&self, ease: EasingFunction) -> f32
    where
        EasingFunction: Fn(f32, f32, f32, f32) -> f32;
    fn shoulder(&self, shoulder_length: f32) -> f32;
    fn invert(&self) -> f32;
    fn piecewise(&self, buckets: &Vec<Range<f32>>) -> f32;
    fn within<Range: RangeBounds<f32>>(&self, range: Range) -> bool;
    fn outside<Range: RangeBounds<f32>>(&self, range: Range) -> bool;
    fn round_to_usize(&self) -> usize;
    fn quantize(&self, resolution: usize) -> f32;
    fn rotate(&self, amount: NormalizedF32) -> f32;
    fn into_ordered_not_nan(self) -> ordered_float::NotNan<f32>;
}

impl F32Extension for f32 {
    fn rescale(&self, input_min: f32, input_max: f32, output_min: f32, output_max: f32) -> f32 {
        // Great explanation of this algorithm here:
        // https://stats.stackexchange.com/questions/281162/scale-a-number-between-a-range/281164
        let input_size = input_max - input_min;
        let normalized = (self - input_min) / input_size;

        let output_size = output_max - output_min;
        (normalized * output_size) + output_min
    }

    fn is_between(&self, min: f32, max: f32) -> bool {
        *self >= min && *self <= max
    }

    fn is_normalized(&self) -> bool {
        self.is_between(0.0, 1.0)
    }

    fn normalize(&self, input_min: f32, input_max: f32) -> f32 {
        self.rescale(input_min, input_max, 0.0, 1.0)
    }

    fn denormalize(&self, output_min: f32, output_max: f32) -> f32 {
        self.rescale(0.0, 1.0, output_min, output_max)
    }

    fn denormalize_to_range(&self, range: &RangeInclusive<f32>) -> f32 {
        self.denormalize(*range.start(), *range.end())
    }

    fn denormalize_symmetrically(&self, radius: f32) -> f32 {
        self.denormalize(-radius, radius)
    }

    fn lerp_w(&self, rect: &Rect) -> f32 {
        rect.lerp_w(*self)
    }

    fn lerp_h(&self, rect: &Rect) -> f32 {
        rect.lerp_h(*self)
    }

    fn times(&self, other: f32) -> f32 {
        self * other
    }

    fn plus(&self, other: f32) -> f32 {
        self + other
    }

    fn minus(&self, other: f32) -> f32 {
        self - other
    }

    fn divided_by(&self, other: f32) -> f32 {
        self / other
    }

    fn clamp(&self, min: f32, max: f32) -> f32 {
        if self > &max {
            return max;
        }

        if self < &min {
            return min;
        }

        *self
    }

    fn normalize_w(&self, rect: &Rect) -> f32 {
        rect.normalize_w(*self)
    }

    fn normalize_h(&self, rect: &Rect) -> f32 {
        rect.normalize_w(*self)
    }

    fn turns_to_radians(&self) -> f32 {
        *self * TAU
    }

    fn jitter(&self, rand: &Rand, amplitude: f32) -> f32 {
        *self + rand.standard_gaussian() * amplitude
    }

    fn normalized_cos(&self) -> f32 {
        self.cos().normalize(-1.0, 1.0)
    }

    fn normalized_sin(&self) -> f32 {
        normalized_sin(*self)
    }

    fn is_integer(&self) -> bool {
        self.floor() == *self
    }

    // Takes a value that goes from min to max and normalizes it.
    // It handles out of bounds values by looping them back into
    // the normalized range. In contrast, regular normalization
    // just lets the value exceed the normalized range.
    //
    // For example:
    //
    // 20.0.normalize(0.0, 10.0); // Returns two.
    // 20.0.loop_normalize(0.0, 10.0) // Returns one.
    fn loop_normalize(&self, min: f32, max: f32) -> f32 {
        let normalized = self.normalize(min, max);
        normalized.looped()
    }

    // Loops self so it stays in the range 0.0 to 1.0.
    fn looped(&self) -> f32 {
        (self % 1.0).abs()
    }

    fn close_to(&self, other: f32) -> bool {
        (self - other).abs() < f32::EPSILON
    }

    fn far_from(&self, other: f32) -> bool {
        !self.close_to(other)
    }

    fn linear_interpolate(&self, start: f32, end: f32) -> f32 {
        let progress = *self;
        (1.0 - progress) * start + progress * end
    }

    // Takes a normalized f32 as input and returns a value that gradually goes up to 1 then comes back down to 0 in a triangle wave shape.
    fn oscillate_triangle(&self) -> f32 {
        let input = self.looped();

        if input < 0.5 {
            // 0 to 0.5
            let progress = input.normalize(0.0, 0.5);
            progress.denormalize(0.0, 1.0)
        } else {
            // 0.5 to 1.0
            let progress = input.normalize(0.5, 1.0);
            progress.denormalize(1.0, 0.0)
        }
    }

    // Like oscillate_triangle, but clips the peak at `max`, resulting
    // in a shape like a flattened mountain.
    fn flattened_triangle(&self, max: NormalizedF32) -> NormalizedF32 {
        self.oscillate_triangle().min(max)
    }

    fn oscillate_eased<EasingFunction>(&self, ease: EasingFunction) -> f32
    where
        EasingFunction: Fn(f32, f32, f32, f32) -> f32,
    {
        // You can also do self.oscillate_triangle().ease_in_out_sine()
        // This method mostly exists as documentation for that pattern.
        self.oscillate_triangle().apply_easing_function(ease)
    }

    // Takes self as the input and outputs a function that has three parts:
    // 1. As self increases from 0 to shoulder_length, the function softly increases
    //    from 0 to 1.
    // 2. As self increases from shoulder_length to 1 - shoulder_length, the
    //    function stays at 1. I call this the plateau.
    // 3. As self increases from 1 - shoulder_length to 1, the function softly
    //    decreases from 1 to 0.
    fn shoulder(&self, shoulder_length: f32) -> f32 {
        let plateau_start = shoulder_length;
        let plateau_end = 1.0 - shoulder_length;
        let plateau_domain = plateau_start..=plateau_end;

        let location = if *self < plateau_start {
            Plateau::Climbing
        } else if *self > plateau_end {
            Plateau::Descending
        } else {
            Plateau::On
        };

        let progress_along_shoulder = match location {
            Plateau::Climbing => self.normalize(0.0, plateau_start).denormalize(0.0, 0.5),
            Plateau::On => return 1.0,
            Plateau::Descending => self.normalize(plateau_end, 1.0).denormalize(0.5, 1.0),
        };

        let circle = Circle {
            radius: 0.5,
            center_x: 0.5,
            center_y: 0.0,
        };

        circle
            .vertical_distance_to_edge(progress_along_shoulder)
            .normalize(0.0, circle.radius)
    }

    fn invert(&self) -> f32 {
        1.0 - *self
    }

    fn piecewise(&self, buckets: &Vec<Range<f32>>) -> f32 {
        let bucket_size = 1.0 / buckets.len() as f32;

        let bucket = buckets.normalized_get(*self);
        let (bucket_min, bucket_max) = (bucket.start, bucket.end);

        // The way our logic is written below, 1.0 gets handled
        // incorrectly. The call to loop_normalize returns 0.0 when
        // self is 1.0, but we want 1.0 to return 1.0 so it gives us
        // the end value of the last bucket. That's why we need this
        // if statement.
        let progress_through_bucket = if self.is_one() {
            1.0
        } else {
            self.loop_normalize(0.0, bucket_size)
        };

        progress_through_bucket.denormalize(bucket_min, bucket_max)
    }

    fn within<Range: RangeBounds<f32>>(&self, range: Range) -> bool {
        range.contains(self)
    }

    fn outside<Range: RangeBounds<f32>>(&self, range: Range) -> bool {
        !range.contains(self)
    }

    fn round_to_usize(&self) -> usize {
        self.round() as usize
    }

    // Takes a normalized f32 and quantizes it to the given resolution
    // while keeping it normalized. For example, calling quantize(10)
    // will quantize a normalized float to the values
    // 0.0, 0.1, 0.2, 0.3, ..., 1.0
    fn quantize(&self, resolution: usize) -> f32 {
        let float_resolution = resolution as f32;
        let expanded = self.times(float_resolution).round();
        expanded.normalize(0.0, float_resolution)
    }

    // Takes a normalized f32 and "rotates" it through the range 0.0..=1.0,
    // For example:
    // 0.0.rotate(0.3) = 0.3
    // 0.5.rotate(0.3) = 0.8
    // 0.7.rotate(0.3) = 1.0
    // 0.9.rotate(0.3) = 0.2
    fn rotate(&self, amount: NormalizedF32) -> f32 {
        self.plus(amount).looped()
    }

    fn into_ordered_not_nan(self) -> ordered_float::NotNan<f32> {
        ordered_float::NotNan::new(self).unwrap()
    }
}

#[test]
fn piecewise_basic() {
    let buckets = vec![0.0..10.0, 0.0..1000.0];

    let actual = 0.0.piecewise(&buckets);
    let expected = 0.0;
    assert!(
        actual.close_to(expected),
        "actual: {actual}, expected: {expected}"
    );
    println!();

    let actual = 0.1.piecewise(&buckets);
    let expected = 2.0;
    assert!(
        actual.close_to(expected),
        "actual: {actual}, expected: {expected}"
    );
    println!();

    let actual = 0.4999999.piecewise(&buckets);
    let expected = 9.999998;
    assert!(
        actual.close_to(expected),
        "actual: {actual}, expected: {expected}"
    );
    println!();

    let actual = 0.5.piecewise(&buckets);
    let expected = 0.0;
    assert!(
        actual.close_to(expected),
        "actual: {actual}, expected: {expected}"
    );
    println!();

    let actual = 0.6.piecewise(&buckets);
    let expected = 200.00005;
    assert!(
        actual.close_to(expected),
        "actual: {actual}, expected: {expected}"
    );
    println!();

    let actual = 1.0.piecewise(&buckets);
    let expected = 1000.0;
    assert!(
        actual.close_to(expected),
        "actual: {actual}, expected: {expected}"
    );
    println!();
}

#[derive(Debug)]
enum Plateau {
    Climbing,
    On,
    Descending,
}

pub fn normalized_sin(x: f32) -> f32 {
    x.sin().normalize(-1.0, 1.0)
}
