use crate::prelude::*;

pub fn create(_params: CreateArtworkParams) -> RegularPolygons {
    regular_polygons()
        .num_repeats(10_000)
        .resolution(24)
        .stroke_weight(0.001)
        .color(
            repeated_colors()
                .color_picker(157, 101, 101, 255)
                .color_picker(81, 113, 142, 255),
        )
        .radius_is_constant_for_each_polygon(false)
        .radius(sine().frequency(1.0).amplitude_range(0.003..=0.019))
        .center(center())
}

fn center() -> impl IntoPointGenerator {
    grid_xy()
        .x_resolution(10)
        .y_resolution(10)
        .map(|point, rand| {
            let x_jitter = rand.zero_to_one().denormalize(0.00, 0.00);
            let y_jitter = rand.zero_to_one().denormalize(0.00, 0.00);
            point + pt2(x_jitter, y_jitter)
        })
}

pub struct CreateArtworkParams<'a> {
    pub app: &'a App,
    pub rand: &'a Rand,
    pub container: &'a Rect,
}
