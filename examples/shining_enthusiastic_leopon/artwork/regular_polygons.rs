use crate::prelude::*;

pub fn regular_polygons() -> RegularPolygons {
    RegularPolygons {
        num_repeats: 1,
        background_color: soft_white(),

        // Generators
        resolution_generator: 3.into_context_generator(),
        stroke_weight_generator: 0.001.into_context_generator(),
        polygon_is_filled_generator: false.into_context_generator(),
        radius_generator: 0.001.into_context_generator(),
        color_generator: soft_black().into_context_generator(),
        center_generator: context_uniform_random_xy().into_context_generator(),
    }
}

fn regular_polygon() -> RegularPolygons {
    regular_polygons()
}

pub struct RegularPolygons {
    num_repeats: usize,
    background_color: Hsl,

    // Generators
    resolution_generator: ContextGenerator<(), usize>,
    center_generator: ContextGenerator<(), Point2>,
    stroke_weight_generator: ContextGenerator<(), f32>,
    color_generator: ContextGenerator<(), Hsl>,
    polygon_is_filled_generator: ContextGenerator<(), bool>,
    radius_generator: ContextGenerator<(), f32>,
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

    pub fn resolution(
        mut self,
        resolution_generator: impl IntoContextGenerator<(), usize>,
    ) -> Self {
        self.resolution_generator = resolution_generator.into_context_generator();
        self
    }

    pub fn center(mut self, center_generator: impl IntoContextGenerator<(), Point2>) -> Self {
        self.center_generator = center_generator.into_context_generator();
        self
    }

    pub fn stroke_weight(
        mut self,
        stroke_weight_generator: impl IntoContextGenerator<(), f32>,
    ) -> Self {
        self.stroke_weight_generator = stroke_weight_generator.into_context_generator();
        self
    }

    pub fn color(mut self, color_generator: impl IntoContextGenerator<(), Hsl>) -> RegularPolygons {
        self.color_generator = color_generator.into_context_generator();
        self
    }

    pub fn polygon_is_filled(
        mut self,
        polygon_is_filled_generator: impl IntoContextGenerator<(), bool>,
    ) -> RegularPolygons {
        self.polygon_is_filled_generator = polygon_is_filled_generator.into_context_generator();
        self
    }

    pub fn radius(
        mut self,
        radius_generator: impl IntoContextGenerator<(), f32>,
    ) -> RegularPolygons {
        self.radius_generator = radius_generator.into_context_generator();
        self
    }
}

impl Artwork for RegularPolygons {
    fn draw(&mut self, params: &DrawParams) {
        let rand = params.rand_that_changes_every_frame();

        let center = self.center_generator.generate(rand, ());
        let resolution = self.resolution_generator.generate(rand, ());
        let stroke_weight = self.stroke_weight_generator.generate(rand, ());

        let path = Path2::regular_polygon(&center, resolution, |_normalized_angle| {
            self.radius_generator.generate(rand, ())
        });

        let color = self.color_generator.generate(rand, ());

        let polygon_is_filled = self.polygon_is_filled_generator.generate(rand, ());

        if polygon_is_filled {
            params.draw.polygon().points(path).color(color);
        } else {
            params
                .draw
                .polyline()
                .stroke_weight(stroke_weight)
                .points(path)
                .color(color);
        }
    }

    fn num_repeats(&self) -> usize {
        self.num_repeats
    }

    fn background_color(&self) -> Hsl {
        self.background_color
    }
}
