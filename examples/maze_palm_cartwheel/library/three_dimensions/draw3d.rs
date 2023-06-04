use crate::prelude::*;
use core::cmp::Ordering;

pub struct Draw3d<'draw> {
    draw: &'draw Draw,
    camera: Camera,
    children: Vec<Child>,
}

impl<'draw> Draw3d<'draw> {
    pub fn new(
        draw: &'draw Draw,
        camera_position: Point3,
        look_at: Point3,
        field_of_view: NumberOfTurns,
    ) -> Draw3d<'draw> {
        let camera = Camera::new(camera_position, look_at, field_of_view.turns_to_radians());

        Draw3d {
            draw,
            camera,
            children: vec![],
        }
    }

    pub fn cube(&mut self, color: Hsla, size: f32, center: Point3) {
        let x = center.x;
        let y = center.y;
        let z = center.z;
        let radius = size / 2.0;

        let projected_center = self.camera.project(center);
        let group_z = projected_center.z;

        let rects: Vec<Child> = vec![
            // bottom
            self.rect_group(
                color,
                [
                    pt3(x - radius, y - radius, z - radius),
                    pt3(x + radius, y - radius, z - radius),
                    pt3(x + radius, y - radius, z + radius),
                    pt3(x - radius, y - radius, z + radius),
                ],
                None,
            ),
            // top
            self.rect_group(
                color,
                [
                    pt3(x - radius, y + radius, z - radius),
                    pt3(x + radius, y + radius, z - radius),
                    pt3(x + radius, y + radius, z + radius),
                    pt3(x - radius, y + radius, z + radius),
                ],
                None,
            ),
            // back
            self.rect_group(
                color,
                [
                    pt3(x - radius, y - radius, z - radius),
                    pt3(x + radius, y - radius, z - radius),
                    pt3(x + radius, y + radius, z - radius),
                    pt3(x - radius, y + radius, z - radius),
                ],
                None,
            ),
            // front
            self.rect_group(
                color,
                [
                    pt3(x - radius, y - radius, z + radius),
                    pt3(x + radius, y - radius, z + radius),
                    pt3(x + radius, y + radius, z + radius),
                    pt3(x - radius, y + radius, z + radius),
                ],
                None,
            ),
            // left
            self.rect_group(
                color,
                [
                    pt3(x - radius, y - radius, z - radius),
                    pt3(x - radius, y + radius, z - radius),
                    pt3(x - radius, y + radius, z + radius),
                    pt3(x - radius, y - radius, z + radius),
                ],
                None,
            ),
            // right
            self.rect_group(
                color,
                [
                    pt3(x + radius, y - radius, z - radius),
                    pt3(x + radius, y + radius, z - radius),
                    pt3(x + radius, y + radius, z + radius),
                    pt3(x + radius, y - radius, z + radius),
                ],
                None,
            ),
        ];

        let group = Child::Group {
            z: group_z,
            children: rects,
        };

        self.children.push(group);
    }

    pub fn rect(&mut self, color: Hsla, corners: [Point3; 4]) {
        let group = self.rect_group(color, corners, None);
        self.children.push(group);
    }

    pub fn polyline(
        &self,
        points: &[Point3],
        color: Hsla,
    ) -> nannou::draw::Drawing<'_, nannou::draw::primitive::Path> {
        let projected_points = points.iter().map(|point| self.camera.project(*point));
        self.draw
            .polyline()
            .stroke_weight(0.005)
            .points(projected_points)
            .color(color)
    }

    fn rect_group(&mut self, color: Hsla, corners: [Point3; 4], z: Option<f32>) -> Child {
        let projected = [
            self.camera.project(corners[0]),
            self.camera.project(corners[1]),
            self.camera.project(corners[2]),
            self.camera.project(corners[3]),
        ];

        let group_z = z.unwrap_or_else(|| projected.iter().map(|point| point.z).average());

        Child::Group {
            z: group_z,
            children: vec![
                Child::Triangle {
                    z: group_z,
                    color,
                    corners: [projected[0], projected[1], projected[2]],
                },
                Child::Triangle {
                    z: group_z,
                    color,
                    corners: [projected[2], projected[3], projected[0]],
                },
            ],
        }
    }

    pub fn finish_drawing(&mut self) {
        Draw3d::finish_drawing_group(self.draw, &mut self.children);
    }

    fn finish_drawing_group(draw: &Draw, children: &mut Vec<Child>) {
        children.sort_by(|a, b| a.compare_z(b));
        children.reverse();

        for child in children {
            Draw3d::finish_drawing_child(draw, child);
        }
    }

    fn finish_drawing_child(draw: &Draw, child: &mut Child) {
        match child {
            Child::Group { z: _, children } => Draw3d::finish_drawing_group(draw, children),
            Child::Triangle {
                z: _,
                color,
                corners,
            } => Draw3d::finish_drawing_triangle(draw, color, corners),
        }
    }

    fn finish_drawing_triangle(draw: &Draw, color: &Hsla, corners: &[Point3; 3]) {
        let corners2d = [corners[0].xy(), corners[1].xy(), corners[2].xy()];

        draw.tri()
            .points(corners2d[0], corners2d[1], corners2d[2])
            .color(*color);
    }
}

impl<'draw> Drop for Draw3d<'draw> {
    fn drop(&mut self) {
        self.finish_drawing();
    }
}

#[derive(Debug)]
enum Child {
    Group {
        z: f32,
        children: Vec<Child>,
    },
    Triangle {
        z: f32,
        color: Hsla,
        corners: [Point3; 3],
    },
}

impl Child {
    fn compare_z(&self, other: &Child) -> Ordering {
        self.z().partial_cmp(&other.z()).unwrap()
    }

    fn z(&self) -> f32 {
        match self {
            Child::Group {
                z,
                children: _children,
            } => *z,
            Child::Triangle {
                z,
                color: _color,
                corners: _corners,
            } => *z,
        }
    }
}
