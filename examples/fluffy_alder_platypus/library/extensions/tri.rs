use crate::prelude::*;
use nannou::geom::Tri;

pub trait TriExtension {
    fn point_opposite_longest_side(&self) -> Point2;
    fn longest_side(&self) -> LineSegment;
    fn sides(&self) -> [LineSegment; 3];
    fn subdivide_once(
        &self,
        rand: &Rand,
        min_skew: NormalizedF32,
        max_skew: NormalizedF32,
    ) -> [Tri<Point2>; 2];
    fn subdivide(
        self,
        rand: &Rand,
        num_subdivisions: usize,
        min_skew: NormalizedF32,
        max_skew: NormalizedF32,
    ) -> Vec<Tri<Point2>>;
}

impl TriExtension for Tri<Point2> {
    fn sides(&self) -> [LineSegment; 3] {
        let vertices: Vec<_> = self.vertices().collect();

        let first_side = [vertices[0], vertices[1]];
        let second_side = [vertices[0], vertices[2]];
        let third_side = [vertices[1], vertices[2]];

        [first_side, second_side, third_side]
    }
    fn longest_side(&self) -> LineSegment {
        let sides = self.sides();
        let mut longest_side_so_far = &sides[0];
        let mut biggest_length_so_far = longest_side_so_far.length2();

        for side in sides[1..=2].iter() {
            let side_length = side.length2();

            if side_length > biggest_length_so_far {
                longest_side_so_far = side;
                biggest_length_so_far = side_length;
            }
        }

        *longest_side_so_far
    }
    fn point_opposite_longest_side(&self) -> Point2 {
        let longest_side = self.longest_side();
        self.vertices()
            .filter(|point| !longest_side.contains(point))
            .last()
            .unwrap()
    }
    // Algorithm from "Aesthetically Pleasing Triangle Subdivision"
    // by Tyler Hobbs:
    // https://tylerxhobbs.com/essays/2017/aesthetically-pleasing-triangle-subdivision
    fn subdivide_once(
        &self,
        rand: &Rand,
        min_skew: NormalizedF32,
        max_skew: NormalizedF32,
    ) -> [Tri<Point2>; 2] {
        if min_skew <= 0.0 || max_skew >= 1.0 {
            panic!("Min and max skew should be between 0 and 1, *not* inclusive.");
        }

        let a = self.point_opposite_longest_side();

        let longest_side = self.longest_side();
        let distance_along_longest_side =
            rand.normalized_gaussian().denormalize(min_skew, max_skew);

        let b = longest_side.interpolate(distance_along_longest_side);

        let c1 = longest_side[0];
        let c2 = longest_side[1];

        let new_triangle_1 = Tri::from_vertices([a, b, c1]).unwrap();
        let new_triangle_2 = Tri::from_vertices([a, b, c2]).unwrap();

        [new_triangle_1, new_triangle_2]
    }
    fn subdivide(
        self,
        rand: &Rand,
        num_subdivisions: usize,
        min_skew: NormalizedF32,
        max_skew: NormalizedF32,
    ) -> Vec<Tri<Point2>> {
        let vector = vec![self];
        vector.subdivide(rand, num_subdivisions, min_skew, max_skew)
    }
}
