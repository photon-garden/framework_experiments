use crate::prelude::*;

pub type LerpablePath2 = DistanceLerpablePath<AdjacentPointsInAPath2>;
pub type LerpablePath3 = DistanceLerpablePath<AdjacentPointsInAPath3>;
pub type LerpableShell = DistanceLerpablePath<AdjacentElementsInAShell>;
pub type LerpableDashedPath2 = DistanceLerpablePath<LerpablePath2>;

pub struct DistanceLerpablePath<Segment>
where
    Segment: LerpableSegment,
{
    pub total_length_of_segments: f32,
    pub segments: Vec<Segment>,
}

impl LerpablePath2 {
    pub fn from_path2(path: Path2) -> LerpablePath2 {
        let segments = path
            .windows(2)
            .map(|adjacent_points| {
                let start = adjacent_points[0];
                let end = adjacent_points[1];
                AdjacentPointsInAPath2(start, end)
            })
            .collect();

        DistanceLerpablePath::new(segments)
    }
}

impl LerpablePath3 {
    pub fn from_path3(path: Path3) -> LerpablePath3 {
        let segments = path
            .windows(2)
            .map(|adjacent_points| {
                let start = adjacent_points[0];
                let end = adjacent_points[1];
                AdjacentPointsInAPath3(start, end)
            })
            .collect();

        DistanceLerpablePath::new(segments)
    }
}

impl LerpableShell {
    pub fn from_shell(shell: Shell) -> LerpableShell {
        let segments = shell
            .windows(2)
            .map(|adjacent_elements| {
                let start = adjacent_elements[0];
                let end = adjacent_elements[1];
                AdjacentElementsInAShell(start, end)
            })
            .collect();

        DistanceLerpablePath::new(segments)
    }
}

impl LerpableDashedPath2 {
    pub fn from_lerpable_paths(lerpable_paths: Vec<LerpablePath2>) -> LerpableDashedPath2 {
        DistanceLerpablePath::new(lerpable_paths)
    }
}

impl<Segment> DistanceLerpablePath<Segment>
where
    Segment: LerpableSegment,
{
    pub fn new(segments: Vec<Segment>) -> DistanceLerpablePath<Segment> {
        let mut total_length_of_segments = 0.0;

        for segment in &segments {
            let length = segment.walking_length_of_lerpable_segment();
            total_length_of_segments += length;
        }

        DistanceLerpablePath {
            total_length_of_segments,
            segments,
        }
    }

    pub fn lerp(&self, progress: NormalizedF32) -> Segment::Point {
        if progress >= 1.0 {
            return self.last_point_in_last_segment();
        }

        let target_distance = progress * self.total_length_of_segments;
        dbg!(progress, target_distance, self.total_length_of_segments);

        let mut distance_traversed = 0.0;
        for segment in &self.segments {
            dbg!(distance_traversed);
            let segment_length = segment.walking_length_of_lerpable_segment();
            dbg!(segment_length);

            let distance_traversed_after_this_segment = distance_traversed + segment_length;
            dbg!(distance_traversed_after_this_segment);
            let desired_point_is_in_this_segment =
                distance_traversed_after_this_segment >= target_distance;

            if desired_point_is_in_this_segment {
                let progress_through_segment = target_distance
                    .normalize(distance_traversed, distance_traversed_after_this_segment);

                return segment.interpolate_between_points_in_segment(progress_through_segment);
            }

            distance_traversed = distance_traversed_after_this_segment;
        }

        self.last_point_in_last_segment()
    }

    pub fn last_point_in_last_segment(&self) -> Segment::Point {
        let last_segment = self.segments.last().unwrap();
        last_segment.end()
    }
}

pub trait LerpableSegment {
    type Point: Clone;

    fn walking_length_of_lerpable_segment(&self) -> f32;
    fn interpolate_between_points_in_segment(&self, progress: f32) -> Self::Point;
    fn end(&self) -> Self::Point;
}

pub struct AdjacentPointsInAPath2(pub Point2, pub Point2);

impl LerpableSegment for AdjacentPointsInAPath2 {
    type Point = Point2;

    fn walking_length_of_lerpable_segment(&self) -> f32 {
        let start = self.0;
        let end = self.1;
        start.distance(end)
    }

    fn interpolate_between_points_in_segment(&self, progress: f32) -> Self::Point {
        let start = self.0;
        let end = self.1;
        start.lerp(end, progress)
    }

    fn end(&self) -> Self::Point {
        self.1
    }
}

pub struct AdjacentPointsInAPath3(pub Point3, pub Point3);

impl LerpableSegment for AdjacentPointsInAPath3 {
    type Point = Point3;

    fn walking_length_of_lerpable_segment(&self) -> f32 {
        let start = self.0;
        let end = self.1;
        start.distance(end)
    }

    fn interpolate_between_points_in_segment(&self, progress: f32) -> Self::Point {
        let start = self.0;
        let end = self.1;
        start.lerp(end, progress)
    }

    fn end(&self) -> Self::Point {
        self.1
    }
}

pub struct AdjacentElementsInAShell(pub ShellElement, pub ShellElement);

impl LerpableSegment for AdjacentElementsInAShell {
    type Point = ShellElement;

    fn walking_length_of_lerpable_segment(&self) -> f32 {
        let start = self.0.midpoint();
        let end = self.1.midpoint();
        start.distance(end)
    }

    fn interpolate_between_points_in_segment(&self, progress: f32) -> Self::Point {
        let start = self.0;
        let end = self.1;
        start.lerp(progress, &end)
    }

    fn end(&self) -> Self::Point {
        self.1
    }
}

impl LerpableSegment for LerpablePath2 {
    type Point = Point2;

    fn walking_length_of_lerpable_segment(&self) -> f32 {
        self.total_length_of_segments
    }

    fn interpolate_between_points_in_segment(&self, progress: f32) -> Self::Point {
        self.lerp(progress)
    }

    fn end(&self) -> Self::Point {
        self.last_point_in_last_segment()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lerp() {
        let path = DistanceLerpablePath::from_path2(vec![pt2(0.0, 0.0), pt2(100.0, 100.0)]);
        let actual = path.lerp(0.5);
        let expected = pt2(50.0, 50.0);
        assert_eq!(actual, expected);

        let path = DistanceLerpablePath::from_path2(vec![
            pt2(0.0, 0.0),
            pt2(50.0, 50.0),
            pt2(100.0, 100.0),
        ]);
        let actual = path.lerp(0.5);
        let expected = pt2(50.0, 50.0);
        assert_eq!(actual, expected);

        let path = DistanceLerpablePath::from_path2(vec![
            pt2(0.0, 0.0),
            pt2(50.0, 50.0),
            pt2(100.0, 100.0),
        ]);
        let actual = path.lerp(0.75);
        let expected = pt2(75.0, 75.0);
        assert_eq!(actual, expected);

        let path =
            DistanceLerpablePath::from_path2(vec![pt2(0.0, 0.0), pt2(50.0, 0.0), pt2(50.0, 50.0)]);
        let actual = path.lerp(0.25);
        let expected = pt2(25.0, 0.0);
        assert_eq!(actual, expected);

        let path =
            DistanceLerpablePath::from_path2(vec![pt2(0.0, 0.0), pt2(50.0, 0.0), pt2(50.0, 50.0)]);
        let actual = path.lerp(0.75);
        let expected = pt2(50.0, 25.0);
        assert_eq!(actual, expected);
    }
}
