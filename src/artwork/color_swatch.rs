use crate::prelude::*;
use geo_booleanop::boolean::BooleanOp;
use itertools::Itertools;

// pub fn new() -> Element {
//     Element::once(|params| {
//         let draw = params.draw;
//         let rand = params.rand;

//         let colors = vec![
//             //
//             soft_black(),
//             // warm_blue_50(),
//             // warm_yellow_50(),
//             // warm_red_50(),
//         ];

//         let mut next_id: usize = 0;

//         let shape_colors = (0..10)
//             .map(|_| {
//                 next_id += 1;
//                 let center = rand.xy();
//                 let radius = 0.1;
//                 let ellipse = Path2::regular_polygon(&center, 64, |_angle| radius);
//                 let color = *rand.element(&colors);
//                 (ellipse, color, next_id)
//             })
//             .collect_vec();
//         // let radius = 0.2;
//         // let resolution = 64;
//         // let shapes = vec![
//         //     (
//         //         Path2::regular_polygon(&pt2(0.2, 0.5), resolution, |_angle| radius),
//         //         warm_blue_50(),
//         //     ),
//         //     (
//         //         Path2::regular_polygon(&pt2(0.3, 0.5), resolution, |_angle| radius),
//         //         warm_red_50(),
//         //     ),
//         // ];

//         for (shape, color, _id) in shape_colors.iter().cloned() {
//             draw.polygon().points(shape).color(color);
//         }

//         draw_intersections(draw, shape_colors);

//         // let mut shape_colors_to_process = shape_colors.clone();
//         // loop {
//         //     if shape_colors_to_process.is_empty() {
//         //         break;
//         //     }

//         //     let outer_shape_color = shape_colors_to_process.first().unwrap().clone();
//         //     let (outer_shape, outer_color, outer_id) = outer_shape_color;

//         //     for index in 0..shape_colors_to_process.len() {
//         //         let inner_shape_color = shape_colors_to_process.get(index).unwrap().clone();
//         //         let (inner_shape, inner_color, inner_id) = inner_shape_color;

//         //         if outer_id == inner_id {
//         //             continue;
//         //         }

//         //         let maybe_intersection = outer_shape.intersections(&inner_shape).take_first();
//         //         if let Some(intersection) = maybe_intersection {
//         //             current_id += 1;
//         //             let intersection_color = outer_color.mix_pigment(&inner_color, 0.5);

//         //             draw.polygon()
//         //                 .points(intersection.clone())
//         //                 .color(intersection_color);

//         //             let new_shape_color = (intersection, intersection_color, current_id);
//         //             shape_colors_to_process.push(new_shape_color);
//         //         }
//         //     }
//         // }
//     })
// }

// fn draw_intersections(draw: &Draw, shape_colors: Vec<(Path2, Hsl, usize)>) {
//     let mut spatial_hash = SpatialHash::new(0.5);
//     for shape_color in shape_colors.iter() {
//         spatial_hash.insert(shape_color);
//     }

//     for outer_shape_color in shape_colors.iter() {
//         let (outer_shape, outer_color, outer_id) = outer_shape_color;
//         let neighbors = spatial_hash.neighbors(outer_shape_color).collect_vec();
//         for (inner_shape, inner_color, inner_id) in neighbors {
//             if outer_id == inner_id {
//                 continue;
//             }

//             let maybe_intersection = outer_shape.intersections(inner_shape).take_first();
//             if let Some(intersection) = maybe_intersection {
//                 let intersection_color = outer_color.mix_pigment(inner_color, 0.5);
//                 draw.polygon()
//                     .points(intersection)
//                     .color(intersection_color);
//             }
//         }
//     }
// }

// impl GetBoundingBox for (Path2, Hsl, usize) {
//     fn bounding_box(&self) -> Rect {
//         self.0.bounding_box()
//     }
// }
