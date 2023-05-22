use crate::prelude::*;
use std::ops::RangeInclusive;

#[derive(Clone)]
pub struct Streamline {
    pub seed_point: Point2,
    pub points: Vec<Point2>,
    pub min_distance_for_seed_points: NormalizedF32,
    pub min_distance_for_growth_points: NormalizedF32,
    pub x_range: RangeInclusive<f32>,
    pub y_range: RangeInclusive<f32>,
}

impl Streamline {
    pub fn from_flow_field<FlowField>(
        other_streamlines: &mut StreamlineCollection,
        min_distance_for_seed_points: NormalizedF32,
        min_distance_for_growth_points: NormalizedF32,
        seed_point: Point2,
        step_size: NormalizedF32,
        angle_at: &FlowField,
        x_range: RangeInclusive<f32>,
        y_range: RangeInclusive<f32>,
    ) -> Option<Streamline>
    where
        FlowField: Fn(&Point2) -> NumberOfTurns,
    {
        let flow_field_iterator_should_continue =
            |point: &Point2| point.is_valid_growth_point(&x_range, &y_range, other_streamlines);

        let points = FlowFieldIterator::iterate_forward_and_backward(
            seed_point,
            step_size,
            angle_at,
            &flow_field_iterator_should_continue,
        );

        if points.len() < 2 {
            return None;
        }

        let new_streamline = Streamline {
            seed_point,
            points,
            min_distance_for_seed_points,
            min_distance_for_growth_points,
            x_range,
            y_range,
        };

        Some(new_streamline)
    }

    pub fn from_points(
        _other_streamlines: &mut StreamlineCollection,
        min_distance_for_seed_points: NormalizedF32,
        min_distance_for_growth_points: NormalizedF32,
        seed_point: Point2,
        points: Vec<Point2>,
        x_range: RangeInclusive<f32>,
        y_range: RangeInclusive<f32>,
    ) -> Streamline {
        Streamline {
            seed_point,
            points,
            min_distance_for_seed_points,
            min_distance_for_growth_points,
            x_range,
            y_range,
        }
    }

    pub fn valid_seed_points_iter<'a>(
        &'a self,
        other_streamlines: &'a StreamlineCollection,
    ) -> impl Iterator<Item = Point2> + 'a {
        self.points
            .as_shell_iter(|_, _| {
                [
                    self.min_distance_for_seed_points,
                    self.min_distance_for_seed_points,
                ]
            })
            .flatten()
            .filter(|point| point.is_valid_seed_point(other_streamlines))
    }

    pub fn someone_elses_seed_point_isnt_too_close_to_me(&self, point: &Point2) -> bool {
        self.points.iter().all(|our_point| {
            let distance = our_point.distance(*point);
            distance >= self.min_distance_for_seed_points
        })
    }

    pub fn someone_elses_growth_point_isnt_too_close_to_me(&self, point: &Point2) -> bool {
        self.points.iter().all(|our_point| {
            let distance = our_point.distance(*point);
            distance >= self.min_distance_for_growth_points
        })
    }
}

trait Point2StreamlineExtension {
    fn is_valid_seed_point(&self, other_streamlines: &StreamlineCollection) -> bool;
    fn is_valid_growth_point(
        &self,
        x_range: &RangeInclusive<f32>,
        y_range: &RangeInclusive<f32>,
        other_streamlines: &StreamlineCollection,
    ) -> bool;
}

impl Point2StreamlineExtension for Point2 {
    fn is_valid_seed_point(&self, other_streamlines: &StreamlineCollection) -> bool {
        if !self.x.is_between(0.0, 1.0) {
            return false;
        }

        if !self.y.is_between(0.0, 1.0) {
            return false;
        }

        match other_streamlines.point_cache.points_near(self) {
            None => true,
            Some(streamlines_and_points) => streamlines_and_points.iter().all(
                |(min_distance_for_seed_points, _min_distance_for_growth_points, point)| {
                    let distance = self.distance(*point);
                    distance >= *min_distance_for_seed_points
                },
            ),
        }
    }

    fn is_valid_growth_point(
        &self,
        x_range: &RangeInclusive<f32>,
        y_range: &RangeInclusive<f32>,
        other_streamlines: &StreamlineCollection,
    ) -> bool {
        if !x_range.contains(&self.x) {
            return false;
        }

        if !y_range.contains(&self.y) {
            return false;
        }

        match other_streamlines.point_cache.points_near(self) {
            None => true,
            Some(streamlines_and_points) => streamlines_and_points.iter().all(
                |(_min_distance_for_seed_points, min_distance_for_growth_points, point)| {
                    let distance = self.distance(*point);
                    distance >= *min_distance_for_growth_points
                },
            ),
        }
    }
}
