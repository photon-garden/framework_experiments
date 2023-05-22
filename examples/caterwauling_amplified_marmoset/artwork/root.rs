use crate::prelude::*;

pub fn create(_params: CreateArtworkParams) -> RegularPolygons {
    // let radius = sine().frequency(1.0).denormalize(0.003, 0.019);
    // let radius = sine().frequency(1.0).denormalize(0.003, 0.019);
    // .crystallize();

    regular_polygons()
        .num_repeats(100)
        .resolution(random_usize(5, 25))
        // .resolution(25)
        .stroke_weight(0.001)
        .color(
            repeated_colors()
                .color_picker(252, 197, 102, 255)
                .color_picker(254, 162, 151, 255)
                .color_picker(164, 222, 251, 255),
        )
        .radius(radius())
        .center(center())
        .polygon_is_filled(flip_coin(0.5))
}

fn center() -> impl IntoSmartGenerator<(), Point2> {
    smart_grid_xy()
        .x_resolution(10)
        .y_resolution(10)
        .x_jitter(0.001)
        .y_jitter(0.000)
        .filter(|_generator, rand, _input, _output| rand.flip_coin(0.5))
}

fn radius() -> impl IntoSmartGenerator<Point2, f32> {
    // smart_sine()
    //     .frequency(1.0)
    //     .denormalize_generator(0.003, 0.019)
    //     .map_input(|xy: Point2| xy.y)
    |_: &Rand, xy: Point2| xy.y.denormalize(0.003, 0.019)
}

pub struct CreateArtworkParams<'a> {
    pub app: &'a App,
    pub rand: &'a Rand,
    pub container: &'a Rect,
}
