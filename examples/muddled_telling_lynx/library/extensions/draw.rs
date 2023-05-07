use crate::prelude::*;
use core::clone::Clone;
use nannou::color::conv::IntoLinSrgba;
use nannou::draw::primitive::Path;
use nannou::draw::primitive::PathStroke;
use nannou::draw::properties::ColorScalar;
use nannou::draw::Drawing;
use nannou::prelude::*;

type Points = Vec<Point2>;

pub trait DrawExtension {
    fn polylines(&self, num_polylines: usize) -> PolylineDrawings<PathStroke>;
    fn turns_around_3d(&self, center: &Point3, x_turns: f32, y_turns: f32, z_turns: f32) -> Draw;
    fn turns_around(&self, center: &Point2, turns: NormalizedF32) -> Draw;
    fn jitter_turns(&self, rand: &Rand, center: &Point3, amount: NormalizedF32) -> Draw;
    fn rotate_with_mouse(&self, app: &App, center: Point3) -> Draw;
    fn cube(
        &self,
        center: NormalizedPoint3,
        size: NormalizedF32,
    ) -> nannou::draw::Drawing<'_, nannou::draw::primitive::Mesh>;
    fn shell3(&self, shell: &Shell3) -> nannou::draw::Drawing<'_, nannou::draw::primitive::Mesh>;
}

impl DrawExtension for Draw {
    fn turns_around_3d(&self, center: &Point3, x_turns: f32, y_turns: f32, z_turns: f32) -> Draw {
        self.translate(*center)
            .x_turns(x_turns)
            .y_turns(y_turns)
            .z_turns(z_turns)
            .translate(-*center)
    }
    fn turns_around(&self, center: &Point2, turns: NormalizedF32) -> Draw {
        let xyz = center.extend(0.0);
        self.translate(xyz).z_turns(turns).translate(-xyz)
    }

    fn jitter_turns(&self, rand: &Rand, center: &Point3, amount: NormalizedF32) -> Draw {
        let x_turns = rand.zero_to_one().denormalize(0.0, amount);
        let y_turns = rand.zero_to_one().denormalize(0.0, amount);
        let z_turns = rand.zero_to_one().denormalize(0.0, amount);
        self.turns_around_3d(center, x_turns, y_turns, z_turns)
    }

    fn rotate_with_mouse(&self, app: &App, center: Point3) -> Draw {
        let x_turns = app.normalized_mouse_y();
        let y_turns = app.normalized_mouse_x();
        self.turns_around_3d(&center, x_turns, y_turns, 0.0)
    }

    fn polylines(&self, num_polylines: usize) -> PolylineDrawings<PathStroke> {
        let polyline_drawings: Vec<Drawing<PathStroke>> =
            (0..num_polylines).map(|_| self.polyline()).collect();

        PolylineDrawings { polyline_drawings }
    }

    fn cube(
        &self,
        center: NormalizedPoint3,
        size: NormalizedF32,
    ) -> nannou::draw::Drawing<'_, nannou::draw::primitive::Mesh> {
        let whd = vec3(size, size, size);
        let cuboid = geom::cuboid::Cuboid::from_xyz_whd(center, whd);
        self.mesh().tris(cuboid.triangles_iter())
    }

    fn shell3(&self, shell: &Shell3) -> nannou::draw::Drawing<'_, nannou::draw::primitive::Mesh> {
        self.mesh().tris(shell.triangles())
    }
}

pub struct PolylineDrawings<'a, T> {
    polyline_drawings: Vec<Drawing<'a, T>>,
}

impl<'a> PolylineDrawings<'a, Path> {
    pub fn color<C>(self, color: C) -> Self
    where
        C: IntoLinSrgba<ColorScalar> + Clone,
    {
        let polyline_drawings: Vec<Drawing<Path>> = self
            .polyline_drawings
            .into_iter()
            .map(|drawing| drawing.color(color.clone()))
            .collect();

        PolylineDrawings { polyline_drawings }
    }
}

impl<'a> PolylineDrawings<'a, PathStroke> {
    pub fn color<C>(self, color: C) -> Self
    where
        C: IntoLinSrgba<ColorScalar> + Clone,
    {
        let polyline_drawings: Vec<Drawing<PathStroke>> = self
            .polyline_drawings
            .into_iter()
            .map(|drawing| drawing.color(color.clone()))
            .collect();

        PolylineDrawings { polyline_drawings }
    }
    pub fn points(self, paths: Vec<Vec<Point2>>) -> PolylineDrawings<'a, Path> {
        let polyline_drawings: Vec<Drawing<Path>> = self
            .polyline_drawings
            .into_iter()
            .zip(paths.into_iter())
            .map(|(drawing, path)| drawing.points(path))
            .collect();

        PolylineDrawings { polyline_drawings }
    }
}
