use crate::prelude::*;

pub fn create(_params: CreateArtworkParams) -> RegularPolygons {
    // let radius = sine().frequency(1.0).denormalize(0.003, 0.019);
    // let radius = sine().frequency(1.0).denormalize(0.003, 0.019);
    // .crystallize();

    regular_polygons()
        .num_repeats(100)
        // .resolution(context_uniform_random_usize(5, 25))
        // .resolution(10)
        .resolution(25)
        .stroke_weight(context_uniform_random_f32().denormalize_generator(0.001, 0.005))
        .color(
            context_looped_hsl()
                .color_picker(252, 197, 102, 255)
                .color_picker(254, 162, 151, 255)
                .color_picker(164, 222, 251, 255),
        )
        .radius(radius())
        .center(
            context_grid_xy()
                .dimensions(4, 4)
                .map_output(|grid_point| grid_point.xy),
            // random_center(),
        )
        .polygon_is_filled(false)
}

fn random_center() -> impl IntoContextGenerator<(), Point2> {
    let max_distance = pt2(0.0, 0.0).distance(pt2(1.0, 1.0));

    context_uniform_random_xy()
        .with_context(vec![], |points, new_point| points.push(new_point))
        .filter(move |params| {
            let new_point = params.output;
            let previous_points = params.context;
            let num_points = previous_points.len();

            let average_distance = if num_points == 0 {
                0.0
            } else {
                previous_points
                    .iter()
                    .map(|neighbor| neighbor.distance(*new_point))
                    .sum::<f32>()
                    .divided_by(num_points as f32)
            };

            let normalized_average_distance = average_distance.normalize(0.0, max_distance);

            let likelihood = normalized_average_distance
                .ease_out_quart() // This repeated ease_out_quart() causes the points to be more clustered.
                .ease_out_quart()
                .invert();

            params.rand.flip_coin(likelihood)
        })
}

// How would I make an image where the colors depend on the integer coordinates of the grid?

// fn grid_center() -> impl IntoContextGenerator<(), Point2> {
//     context_grid_xy()
//         .with_context(vec![], |points, point| points.push(point))
//         .filter(move |params| {
//             let xy = params.output;
//             let other_points = params.context;
//             let num_points = other_points.len();

//             let average_distance = if num_points == 0 {
//                 0.0
//             } else {
//                 other_points
//                     .iter()
//                     .map(|neighbor| neighbor.distance(*xy))
//                     .sum::<f32>()
//                     .divided_by(num_points as f32)
//             };

//             let likelihood = average_distance
//                 .normalize(0.0, max_distance)
//                 .ease_out_quart() // This repeated ease_out_quart() causes the points to be more clustered.
//                 .ease_out_quart()
//                 .invert();

//             params.rand.flip_coin(likelihood)
//         })
// }

fn radius() -> impl IntoContextGenerator<(), f32> {
    // sine()
    //     .frequency(1.0)
    //     .denormalize_generator(0.003, 0.019)
    //     .map_input(|xy: Point2| xy.y)
    // |_: &Rand, xy: Point2| xy.y.denormalize(0.003, 0.019)
    context_uniform_random_f32()
        .denormalize_generator(0.005, 0.015)
        .crystallize()
}

pub struct CreateArtworkParams<'a> {
    pub app: &'a App,
    pub rand: &'a Rand,
    pub container: &'a Rect,
}
