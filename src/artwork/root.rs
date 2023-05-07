use crate::prelude::*;

pub fn create(_params: CreateArtworkParams) -> RegularPolygons {
    regular_polygons()
        .num_repeats(1000)
        .resolution(8)
        .stroke_weight(0.001)
        .color(
            repeated_colors()
                .color_picker(252, 197, 102, 255)
                .color_picker(254, 162, 151, 255),
        )
        .radius_is_constant_for_each_polygon(false)
        .radius(sine().frequency(1.0).amplitude(0.003..=0.019))
        .center(center())
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
