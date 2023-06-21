use ordered_float::NotNan;

use crate::prelude::*;

pub struct Circle3 {}

impl Circle3 {
    // How to compute a circle in three dimensions, given two vectors that define a plane.
    // https://math.stackexchange.com/questions/1184038/what-is-the-equation-of-a-general-circle-in-3-d-space
    pub fn points<GetRadius>(
        center: Point3,
        perpendicular_to_plane: Vec3,
        resolution: usize,
        get_radius: GetRadius,
    ) -> Path3
    where
        GetRadius: Fn(ProgressAroundRing) -> NormalizedF32,
    {
        // first_orthonormal, second_orthonormal, and perpendicular_to_plane form an orthonormal basis:
        // they're all perpendicular to each other and normalized. If you think of the plane
        // parallel to first_orthonormal and second_orthonormal, perpendicular_to_plane is perpendicular to
        // that plane.
        let (first_orthonormal, second_orthonormal) = perpendicular_to_plane.any_orthonormal_pair();

        let mut point_with_min_y: Option<Point3> = None;

        let mut points = Vec::with_capacity(resolution);
        let placeholder_radius = 1.0;
        for progress in zero_to_one(resolution) {
            let angle = progress.turns_to_radians();

            let a = first_orthonormal
                .times(placeholder_radius)
                .times(angle.cos());
            let b = second_orthonormal
                .times(placeholder_radius)
                .times(angle.sin());

            let point = a + b + center;

            match point_with_min_y {
                Some(point_with_min_y_so_far) => {
                    if point.y < point_with_min_y_so_far.y {
                        point_with_min_y = Some(point);
                    }
                }
                None => {
                    point_with_min_y = Some(point);
                }
            }

            points.push(point);
        }

        let from_center_to_point_with_min_y = center.between(point_with_min_y.unwrap());

        // Sometimes one of the orthonormals flips for some unknown reason,
        // meaning the points in our circle get generated in a different order.
        // To keep a visually stable ordering regardless of what shenanigans
        // the orthonormals get up to, we compute the angle between two vectors.
        // The first is center -> point. The second is center -> point with min y.
        //
        // glam's default angle_between method doesn't give us a signed angle,
        // which we need to assure that the points end up in the proper order,
        // so we use a different method.
        points.sort_by_key(|point| {
            let from_center_to_point = center.between(*point);
            let angle = from_center_to_point_with_min_y
                .signed_angle_between(from_center_to_point, perpendicular_to_plane);
            NotNan::new(angle).unwrap()
        });

        // We have to do this dance with the placeholder radius because on the first pass,
        // we compute points in one order, but when they're sorted, they're in a different
        // order. If we call get_radius on the first pass, its argument is wrong because
        // of the different ordering.
        for (progress, point) in points.iter_mut().enumerate_normalized() {
            let radius = get_radius(progress);
            // Our placeholder radius is 1.0, so center.between(point) is
            // normalized and we can just multiply by radius to get a vector
            // of length radius.
            let between = center.between(*point).times(radius);
            *point = center + between;
        }

        points
    }

    pub fn point(
        center: Point3,
        perpendicular_to_plane: Vec3,
        radius: NormalizedF32,
        turns: NumberOfTurns,
    ) -> Point3 {
        let (first_orthonormal, second_orthonormal) = perpendicular_to_plane.any_orthonormal_pair();

        let angle = turns.turns_to_radians();
        let a = first_orthonormal.times(radius).times(angle.cos());
        let b = second_orthonormal.times(radius).times(angle.sin());

        a + b + center
    }
}
