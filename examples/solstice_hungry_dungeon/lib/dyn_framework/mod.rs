use crate::prelude::extensions::f32::*;
use crate::prelude::pt2;

use f32_generator::*;
use point_generator::*;
use regular_polygons::*;
use signal_generator::*;
use usize_generator::*;

mod color_generator;
pub use color_generator::*;

mod drawing;
pub use drawing::*;

mod f32_generator;
pub use f32_generator::*;

mod generator;
pub use generator::*;

mod point_generator;
pub use point_generator::*;

mod regular_polygons;
pub use regular_polygons::*;

mod signal_generator;
pub use signal_generator::*;

mod usize_generator;
pub use usize_generator::*;

pub fn draw() -> RegularPolygons {
    regular_polygons()
        .repeat(10_000)
        .resolution(16)
        .stroke_weight(0.001)
        .color(
            repeated_colors()
                .color_picker(157, 101, 101, 255)
                .color_picker(81, 113, 142, 255),
        )
        .radius_is_constant_for_each_polygon(false)
        .radius(sine().frequency(2.0).amplitude_range(0.003..=0.012))
        .center(
            grid_xy()
                .x_resolution(10)
                .y_resolution(10)
                .map(|point, rand| {
                    let x_jitter = rand.zero_to_one().denormalize(0.00, 0.00);
                    let y_jitter = rand.zero_to_one().denormalize(0.00, 0.00);
                    point + pt2(x_jitter, y_jitter)
                }),
        )
}
