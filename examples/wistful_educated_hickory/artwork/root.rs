use crate::prelude::*;

pub fn create(_params: CreateArtworkParams) -> RegularPolygons {
    regular_polygons()
        .num_repeats(100)
        .resolution(random_usize(5, 25))
        .stroke_weight(0.001)
        .color(
            repeated_colors()
                .color_picker(252, 197, 102, 255)
                .color_picker(254, 162, 151, 255)
                .color_picker(164, 222, 251, 255),
        )
        .radius_is_constant_for_each_polygon(false)
        .radius(sine().frequency(1.0).denormalize(0.003, 0.019))
        .center(center())
        .polygon_is_filled(true)
}

fn center() -> impl IntoPointGenerator {
    grid_xy()
        .x_resolution(10)
        .y_resolution(10)
        .x_jitter(0.001)
        .y_jitter(0.000)
}

pub struct CreateArtworkParams<'a> {
    pub app: &'a App,
    pub rand: &'a Rand,
    pub container: &'a Rect,
}
