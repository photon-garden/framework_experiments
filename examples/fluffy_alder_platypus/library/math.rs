use crate::prelude::*;

pub const PI_HALVES: f32 = PI / 2.0;

pub fn golden_ratio() -> f32 {
    (1.0 + 5.0.sqrt()) / 2.0
}

// The return value ranges from 0 to different numbers. The small the standard deviation,
// the higher the maximum return value.
pub fn gaussian(x: f32, mean: f32, standard_deviation: f32) -> f32 {
    let sqrt_of_tau: f32 = TAU.sqrt();

    let lhs = {
        let numerator = 1.0;
        let denominator = standard_deviation * sqrt_of_tau;
        numerator / denominator
    };

    let exponent = {
        let a = -0.5;
        let b = (x - mean) / standard_deviation;
        let b_squared = b.powi(2);
        a * b_squared
    };

    let rhs = std::f32::consts::E.powf(exponent);

    lhs * rhs
}

// This is normalized so the return value is always between 0 and 1.
pub fn normalized_gaussian(x: f32, mean: f32, standard_deviation: f32) -> f32 {
    // The max value of the Gaussian distribution always happens at the mean,
    // so we start by computing that.
    let max_g = gaussian(mean, mean, standard_deviation);
    let g = gaussian(x, mean, standard_deviation);
    g / max_g
}

pub fn polar_to_cartesian(radius: f32, angle: NumberOfTurns) -> Point2 {
    let turns = angle.turns_to_radians();

    let x = radius * turns.sin();
    let y = radius * turns.cos();

    pt2(x, y)
}
