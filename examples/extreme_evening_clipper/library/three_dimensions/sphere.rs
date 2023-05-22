use crate::prelude::*;

pub struct Sphere {
    pub center_x: f32,
    pub center_y: f32,
    pub center_z: f32,

    pub radius: f32,
}

impl Sphere {
    // From http://extremelearning.com.au/how-to-evenly-distribute-points-on-a-sphere-more-effectively-than-the-canonical-fibonacci-lattice/
    pub fn fibonacci_grid(
        center: Point3,
        radius: f32,
        resolution: usize,
    ) -> impl Iterator<Item = Point3> {
        let golden_ratio = golden_ratio();
        (0..resolution).into_iter().map(move |integer_index| {
            let i = integer_index as f32;
            let resolution = resolution as f32;

            let theta = TAU.times(i).divided_by(golden_ratio);
            let phi = (1.0 - 2.0 * (i + 0.5) / resolution).acos();
            let x = theta.cos() * phi.sin();
            let y = theta.sin() * phi.sin();
            let z = phi.cos();

            pt3(x, y, z).times(radius).plus(&center)
        })
    }

    // This function returns the distance between
    // 1. the xy plane that crosses the center of the sphere, and
    // 2. the edge of the sphere in the z direction.
    //
    // The formula is similar to the circle formula.
    pub fn z_distance_to_edge(&self, x: f32, y: f32) -> f32 {
        let radius_squared = self.radius.powi(2);

        let x_distance_from_center = x - self.center_x;
        let x_distance_from_center_squared = x_distance_from_center.powi(2);

        let y_distance_from_center = y - self.center_y;
        let y_distance_from_center_squared = y_distance_from_center.powi(2);

        // Occasionally floating point shenanigans cause this to fall below zero.
        // If that happens, .sqrt() returns NaN which isn't what we want. Instead,
        // make sure we're always running sqrt() on a number that's greater than
        // or equal to zero.
        (radius_squared - x_distance_from_center_squared - y_distance_from_center_squared)
            .max(0.0)
            .sqrt()
    }
}
