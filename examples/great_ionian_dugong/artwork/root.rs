use std::{cell::RefCell, rc::Rc};

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
            looped_colors()
                .color_picker(252, 197, 102, 255)
                .color_picker(254, 162, 151, 255)
                .color_picker(164, 222, 251, 255),
        )
        .radius(radius())
        .center(center())
        .polygon_is_filled(flip_coin(0.5))
}

fn center() -> impl IntoGenerator<(), Point2> {
    let points: Rc<RefCell<Vec<Point2>>> = Rc::new(RefCell::new(vec![]));
    let closure_points = points.clone();
    let max_distance = pt2(0.0, 0.0).distance(pt2(1.0, 1.0));

    grid_xy()
        .x_resolution(10)
        .y_resolution(10)
        .x_jitter(0.001)
        .y_jitter(0.000)
        .filter(move |params| {
            let xy = params.output;
            let num_points = closure_points.borrow().len();

            let average_distance = if num_points == 0 {
                0.0
            } else {
                closure_points
                    .borrow()
                    .iter()
                    .map(|neighbor| neighbor.distance(*xy))
                    .sum::<f32>()
                    .divided_by(num_points as f32)
            };

            let likelihood = average_distance.normalize(0.0, max_distance).invert();
            params.rand.flip_coin(likelihood)
        })
        .save_outputs(points)
}

fn radius() -> impl IntoGenerator<(), f32> {
    // sine()
    //     .frequency(1.0)
    //     .denormalize_generator(0.003, 0.019)
    //     .map_input(|xy: Point2| xy.y)
    // |_: &Rand, xy: Point2| xy.y.denormalize(0.003, 0.019)
    random_f32().denormalize_generator(0.01, 0.03).crystallize()
}

pub struct CreateArtworkParams<'a> {
    pub app: &'a App,
    pub rand: &'a Rand,
    pub container: &'a Rect,
}
