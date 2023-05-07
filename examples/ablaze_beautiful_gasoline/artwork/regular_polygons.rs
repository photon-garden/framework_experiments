use super::*;
use crate::prelude::*;
use std::cell::{Cell, RefCell};
use std::time::Instant;

pub fn regular_polygons() -> RegularPolygons {
    RegularPolygons {
        resolution_generator: 3.into_usize_generator(),
        radius_generator: 0.001.into_signal_generator(),
        center_generator: pt2(0.0, 0.0).into_point_generator(),
        radius_is_constant_for_each_polygon: true,
        stroke_weight_generator: 0.001.into_f32_generator(),
        color_generator: Box::<OneColorGenerator>::new(soft_black().into()),
        num_repeats: 1,
        background_color: soft_white(),
    }
}

fn regular_polygon() -> RegularPolygons {
    regular_polygons()
}

pub struct RegularPolygons {
    resolution_generator: BoxedGenerator<usize>,
    radius_generator: BoxedSignalGenerator<f32>,
    radius_is_constant_for_each_polygon: bool,
    center_generator: BoxedGenerator<Point2>,
    stroke_weight_generator: BoxedGenerator<f32>,
    color_generator: BoxedGenerator<Hsl>,
    num_repeats: usize,
    background_color: Hsl,
}

impl RegularPolygons {
    pub fn background_color(mut self, background_color: Hsl) -> Self {
        self.background_color = background_color;
        self
    }

    pub fn num_repeats(mut self, num_repeats: usize) -> Self {
        self.num_repeats = num_repeats;
        self
    }

    pub fn resolution(mut self, resolution_generator: impl IntoBoxedUsizeGenerator) -> Self {
        self.resolution_generator = resolution_generator.into_usize_generator();
        self
    }

    pub fn radius(mut self, radius_generator: impl IntoF32SignalGenerator) -> Self {
        self.radius_generator = radius_generator.into_signal_generator();
        self
    }

    pub fn center(mut self, center_generator: impl IntoPointGenerator) -> Self {
        self.center_generator = center_generator.into_point_generator();
        self
    }

    pub fn stroke_weight(mut self, stroke_weight_generator: impl IntoF32Generator) -> Self {
        self.stroke_weight_generator = stroke_weight_generator.into_f32_generator();
        self
    }

    pub fn radius_is_constant_for_each_polygon(
        mut self,
        radius_is_constant_for_each_polygon: bool,
    ) -> Self {
        self.radius_is_constant_for_each_polygon = radius_is_constant_for_each_polygon;
        self
    }

    pub fn color(mut self, color_generator: impl IntoBoxedColorGenerator) -> RegularPolygons {
        self.color_generator = color_generator.into_boxed_color_generator();
        self
    }
}

impl Artwork for RegularPolygons {
    fn draw(&mut self, params: &DrawParams) {
        let rand = params.rand_that_changes_every_frame();

        let center = self.center_generator.generate(rand);
        let resolution = self.resolution_generator.generate(rand);
        let stroke_weight = self.stroke_weight_generator.generate(rand);

        let path = if self.radius_is_constant_for_each_polygon {
            let radius = self
                .radius_generator
                .generate(rand, params.progress_through_whole_drawing);
            crate::prelude::Path2::regular_polygon(&center, resolution, |_| radius)
        } else {
            crate::prelude::Path2::regular_polygon(&center, resolution, |normalized_angle| {
                self.radius_generator.generate(rand, normalized_angle)
            })
        };

        let color = self.color_generator.generate(rand);

        params
            .draw
            .polyline()
            // .polygon()
            .stroke_weight(stroke_weight)
            .points(path)
            .color(color);
    }

    fn num_repeats(&self) -> usize {
        self.num_repeats
    }

    fn background_color(&self) -> Hsl {
        self.background_color
    }
}
