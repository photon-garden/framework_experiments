use crate::prelude::*;

pub struct Circle {
    pub center_x: f32,
    pub center_y: f32,
    pub radius: f32,
}

impl Circle {
    // This function returns the distance between the
    // horizontal line crossing the center of the circle
    // and the top edge of the circle.
    //
    // Here's the mathematical reasoning, starting from
    // the definition of a circle on a Cartesian plane.
    //
    // (x - center_x)^2 + (y - center_y)^2 = r^2
    // (y - center_y)^2 = r^2 - (x - center_x)^2
    // y - center_y = sqrt(r^2 - (x - center_x)^2)
    // y = sqrt(r^2 - (x - center_x)^2) + center_y
    //
    // Then we drop `+ center_y` because we don't want the
    // actual y value, just the vertical distance from the
    // center line to the edge of the circle.
    //
    // vertical_distance = sqrt(r^2 + (x - center_x)^2)
    pub fn vertical_distance_to_edge(&self, x: f32) -> f32 {
        let radius_squared = self.radius.powi(2);

        let x_distance_from_center = x - self.center_x;
        let x_distance_from_center_squared = x_distance_from_center.powi(2);

        // Occasionally floating point shenanigans cause this to fall below zero.
        // If that happens, .sqrt() returns NaN which isn't what we want. Instead,
        // make sure we're always running sqrt() on a number that's greater than
        // or equal to zero.
        (radius_squared - x_distance_from_center_squared)
            .max(0.0)
            .sqrt()
    }

    pub fn left_x(&self) -> f32 {
        self.center_x - self.radius
    }

    pub fn right_x(&self) -> f32 {
        self.center_x + self.radius
    }

    pub fn left(&self) -> Point2 {
        pt2(self.left_x(), self.center_y)
    }

    pub fn right(&self) -> Point2 {
        pt2(self.right_x(), self.center_y)
    }
}
