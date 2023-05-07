use crate::prelude::*;
use itertools::Itertools;

pub struct FlowFieldIterator<'a, FlowField, ShouldContinue>
where
    FlowField: Fn(&Point2) -> NumberOfTurns,
    ShouldContinue: Fn(&Point2) -> bool,
{
    // other_streamlines: &'a [Streamline],
    pub current_point: Point2,
    pub step_size: NormalizedF32,
    pub move_forward: bool,
    pub angle_at: &'a FlowField,
    pub should_continue: &'a ShouldContinue,
}

impl<'a, FlowField, ShouldContinue> FlowFieldIterator<'a, FlowField, ShouldContinue>
where
    FlowField: Fn(&Point2) -> NumberOfTurns,
    ShouldContinue: Fn(&Point2) -> bool,
{
    pub fn iterate_forward_and_backward(
        start_point: Point2,
        step_size: f32,
        angle_at: &FlowField,
        should_continue: &ShouldContinue,
    ) -> Vec<Point2> {
        // Goes from the middle (the seed point) to the end.
        let mut forward_iter = FlowFieldIterator {
            move_forward: true,
            current_point: start_point,
            should_continue,
            step_size,
            angle_at,
        };
        forward_iter.next(); // Skip the first value (the seed point). If we don't skip this, it'll be duplicated in the final vector.
        let mut forward = forward_iter.collect_vec();

        // Goes from the middle (the seed point) to the beginning.
        let mut backward = FlowFieldIterator {
            move_forward: false,
            current_point: start_point,
            should_continue,
            step_size,
            angle_at,
        }
        .collect_vec();

        // Now backward goes from beginning to middle.
        backward.reverse();

        // Now backward goes from beginning to end.
        backward.append(&mut forward);

        backward
    }
}

impl<'a, FlowField, ShouldContinue> Iterator for FlowFieldIterator<'a, FlowField, ShouldContinue>
where
    FlowField: Fn(&Point2) -> NumberOfTurns,
    ShouldContinue: Fn(&Point2) -> bool,
{
    type Item = Point2;

    fn next(&mut self) -> Option<Self::Item> {
        let current_point = self.current_point;

        // if !current_point.is_valid_growth_point(self.other_streamlines) {
        //     return None;
        // }

        let should_continue = self.should_continue;

        if !should_continue(&current_point) {
            return None;
        }

        let angle_at = self.angle_at;

        let mut angle = angle_at(&self.current_point);

        if !self.move_forward {
            angle += 0.5;
        }

        let step = Vec2::from_angle(angle) * self.step_size;
        let next_point = current_point + step;

        self.current_point = next_point;

        // Notice how we return the current_point variable here, not self.current_point.
        // At this point in the function, current_point is actually old -- we've already
        // figured out what the next value will be.
        //
        // But we need to do it this way because we want the first call to next to return
        // the point that the FlowFieldIterator was initialized with.
        Some(current_point)
    }
}
