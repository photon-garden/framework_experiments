use crate::prelude::*;
use nannou::geom::Tri;

pub trait TrianglesExtension {
    fn subdivide(
        self,
        rand: &Rand,
        num_subdivisions: usize,
        min_skew: NormalizedF32,
        max_skew: NormalizedF32,
    ) -> Vec<Tri<Point2>>;
}

impl TrianglesExtension for Vec<Tri<Point2>> {
    fn subdivide(
        self,
        rand: &Rand,
        num_subdivisions: usize,
        min_skew: NormalizedF32,
        max_skew: NormalizedF32,
    ) -> Vec<Tri<Point2>> {
        if num_subdivisions == 0 {
            return self;
        }

        let mut to_subdivide = self;

        if num_subdivisions > to_subdivide.capacity() {
            let additional_allocations_required = num_subdivisions - to_subdivide.capacity();
            to_subdivide.reserve(additional_allocations_required);
        }

        let mut already_subdivided = Vec::with_capacity(num_subdivisions * 2);

        for _ in 0..num_subdivisions {
            let index = rand.index(&to_subdivide);
            let triangle = to_subdivide.swap_remove(index);
            let new_triangles = triangle.subdivide_once(rand, min_skew, max_skew);

            to_subdivide.push(new_triangles[0]);
            to_subdivide.push(new_triangles[1]);

            already_subdivided.push(triangle);
        }

        already_subdivided.append(&mut to_subdivide);

        already_subdivided
    }
}
