use crate::prelude::*;
use std::ops::RangeInclusive;

mod flow_field_iterator;
pub use flow_field_iterator::*;

mod streamline;
pub use streamline::Streamline;

mod streamline_collection;
pub use streamline_collection::*;

mod point_cache;
pub use point_cache::*;

pub fn new<FlowField, GetMinDistanceForSeedPoints, GetMinDistanceForGrowthPoints, GetXAndYRange>(
    starting_seed_point: Point2,
    get_min_distance_for_seed_points: GetMinDistanceForSeedPoints,
    get_min_distance_for_growth_points: GetMinDistanceForGrowthPoints,
    step_size: NormalizedF32,
    flow_field: &FlowField,
    get_x_and_y_ranges: &GetXAndYRange,
) -> Vec<Streamline>
where
    FlowField: Fn(&Point2) -> NumberOfTurns,
    GetMinDistanceForSeedPoints: Fn(&Point2) -> NormalizedF32,
    GetMinDistanceForGrowthPoints: Fn(&Point2) -> NormalizedF32,
    GetXAndYRange: Fn(&Point2) -> [RangeInclusive<f32>; 2],
{
    let mut streamlines = StreamlineCollection::new(vec![]);

    let [starting_x_range, starting_y_range] = get_x_and_y_ranges(&starting_seed_point);
    let starting_streamline = Streamline::from_flow_field(
        &mut streamlines,
        get_min_distance_for_seed_points(&starting_seed_point),
        get_min_distance_for_growth_points(&starting_seed_point),
        starting_seed_point,
        step_size,
        flow_field,
        starting_x_range,
        starting_y_range,
    )
    .expect("Couldn't make the first seed line.");

    streamlines.vector.push(starting_streamline);

    // starting_streamline.points = starting_streamline.points.smooth(0.25, 3);

    let mut index = 0;

    loop {
        match streamlines.vector.get(index) {
            None => break,
            Some(streamline) => {
                index += 1;

                let seed_points: Vec<_> = streamline.valid_seed_points_iter(&streamlines).collect();

                // if seed_points.len() > 1 {
                //     params.rand.shuffle(&mut seed_points);
                // }

                for seed_point in seed_points {
                    // let min_distance_for_seed_points =
                    //     sine_of_distance_from_origin(&seed_point).denormalize(0.001, 0.03);
                    // let min_distance_for_growth_points = min_distance_for_seed_points * 0.9;

                    let [x_range, y_range] = get_x_and_y_ranges(&seed_point);

                    let maybe_new_streamline = Streamline::from_flow_field(
                        &mut streamlines,
                        get_min_distance_for_seed_points(&seed_point),
                        get_min_distance_for_growth_points(&seed_point),
                        seed_point,
                        step_size,
                        &flow_field,
                        x_range,
                        y_range,
                    );

                    if let Some(new_streamline) = maybe_new_streamline {
                        for point in new_streamline.points.iter() {
                            streamlines.point_cache.add_point(
                                new_streamline.min_distance_for_seed_points,
                                new_streamline.min_distance_for_growth_points,
                                *point,
                            );
                        }
                        // new_streamline.points = new_streamline.points.smooth(0.25, 3);
                        streamlines.vector.push(new_streamline);
                        continue;
                    }
                }
            }
        }
    }

    streamlines.vector
}

// pub fn new_iterator<
//     FlowField,
//     GetMinDistanceForSeedPoints,
//     GetMinDistanceForGrowthPoints,
//     GetXAndYRange,
// >(
//     starting_seed_point: Point2,
//     get_min_distance_for_seed_points: GetMinDistanceForSeedPoints,
//     get_min_distance_for_growth_points: GetMinDistanceForGrowthPoints,
//     step_size: NormalizedF32,
//     flow_field: &FlowField,
//     get_x_and_y_ranges: &GetXAndYRange,
// ) -> Vec<Streamline>
// where
//     FlowField: Fn(&Point2) -> NumberOfTurns,
//     GetMinDistanceForSeedPoints: Fn(&Point2) -> NormalizedF32,
//     GetMinDistanceForGrowthPoints: Fn(&Point2) -> NormalizedF32,
//     GetXAndYRange: Fn(&Point2) -> [RangeInclusive<f32>; 2],
// {
//     let [starting_x_range, starting_y_range] = get_x_and_y_ranges(&starting_seed_point);
//     let starting_streamline = Streamline::from_flow_field(
//         &[],
//         get_min_distance_for_seed_points(&starting_seed_point),
//         get_min_distance_for_growth_points(&starting_seed_point),
//         starting_seed_point,
//         step_size,
//         flow_field,
//         starting_x_range,
//         starting_y_range,
//     )
//     .expect("Couldn't make the first seed line.");

//     // starting_streamline.points = starting_streamline.points.smooth(0.25, 3);

//     let mut streamlines = vec![starting_streamline];

//     let mut index = 0;

//     streamlines
// }

// pub struct StreamlineIterator<
//     'a,
//     FlowField,
//     GetMinDistanceForSeedPoints,
//     GetMinDistanceForGrowthPoints,
//     GetXAndYRange,
// > where
//     FlowField: Fn(&Point2) -> NumberOfTurns,
//     GetMinDistanceForSeedPoints: Fn(&Point2) -> NormalizedF32,
//     GetMinDistanceForGrowthPoints: Fn(&Point2) -> NormalizedF32,
//     GetXAndYRange: Fn(&Point2) -> [RangeInclusive<f32>; 2],
// {
//     index: usize,
//     streamlines: Vec<Streamline>,
//     starting_seed_point: Point2,
//     get_min_distance_for_seed_points: GetMinDistanceForSeedPoints,
//     get_min_distance_for_growth_points: GetMinDistanceForGrowthPoints,
//     step_size: NormalizedF32,
//     flow_field: &'a FlowField,
//     get_x_and_y_ranges: &'a GetXAndYRange,
// }

// impl<'a, FlowField, GetMinDistanceForSeedPoints, GetMinDistanceForGrowthPoints, GetXAndYRange>
//     Iterator
//     for StreamlineIterator<
//         'a,
//         FlowField,
//         GetMinDistanceForSeedPoints,
//         GetMinDistanceForGrowthPoints,
//         GetXAndYRange,
//     >
// where
//     FlowField: Fn(&Point2) -> NumberOfTurns,
//     GetMinDistanceForSeedPoints: Fn(&Point2) -> NormalizedF32,
//     GetMinDistanceForGrowthPoints: Fn(&Point2) -> NormalizedF32,
//     GetXAndYRange: Fn(&Point2) -> [RangeInclusive<f32>; 2],
// {
//     type Item = &'a Streamline;

//     fn next(&mut self) -> Option<Self::Item> {
//         loop {
//             match self.streamlines.get(self.index) {
//                 None => return None,
//                 Some(streamline) => {
//                     self.index += 1;

//                     let seed_points: Vec<_> = streamline
//                         .valid_seed_points_iter(&self.streamlines)
//                         .collect();

//                     // if seed_points.len() > 1 {
//                     //     params.rand.shuffle(&mut seed_points);
//                     // }

//                     for seed_point in seed_points {
//                         // let min_distance_for_seed_points =
//                         //     sine_of_distance_from_origin(&seed_point).denormalize(0.001, 0.03);
//                         // let min_distance_for_growth_points = min_distance_for_seed_points * 0.9;

//                         let [x_range, y_range] = self.get_x_and_y_ranges(&seed_point);

//                         let maybe_new_streamline = Streamline::from_flow_field(
//                             &self.streamlines,
//                             self.get_min_distance_for_seed_points(&seed_point),
//                             self.get_min_distance_for_growth_points(&seed_point),
//                             seed_point,
//                             self.step_size,
//                             &self.flow_field,
//                             x_range,
//                             y_range,
//                         );

//                         if let Some(new_streamline) = maybe_new_streamline {
//                             // new_streamline.points = new_streamline.points.smooth(0.25, 3);
//                             self.streamlines.push(new_streamline);
//                             return Some(&new_streamline);
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }
