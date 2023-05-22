use crate::prelude::*;

#[derive(Clone)]
pub struct Camera {
    camera_matrix: Mat4,
    perspective_matrix: Mat4,
}

impl Camera {
    pub fn new(position: Point3, look_at: Point3, _field_of_view: f32) -> Self {
        let camera_matrix = Mat4::look_at_lh(position, look_at, Vec3::Y);
        // let perspective_matrix = Mat4::perspective_infinite_lh(field_of_view, 1.0, 0.0);
        let perspective_matrix = Mat4::orthographic_lh(-1.0, 1.0, -1.0, 1.0, 0.0, 1.0);

        Camera {
            camera_matrix,
            perspective_matrix,
        }
    }

    pub fn project(&self, point3: Point3) -> Point3 {
        let camera_space = self.camera_matrix.transform_point3(point3);
        let with_perspective = self.perspective_matrix.project_point3(camera_space);
        pt3(
            with_perspective.x.normalize(-1.0, 1.0),
            with_perspective.y.normalize(-1.0, 1.0),
            with_perspective.z.normalize(-1.0, 1.0),
        )
    }

    pub fn project_into_xy(&self, point3: Point3) -> Point2 {
        let projected = self.project(point3);
        pt2(projected.x, projected.y)
    }
}
