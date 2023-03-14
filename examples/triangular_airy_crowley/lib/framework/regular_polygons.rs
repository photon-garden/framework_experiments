use super::*;
use crate::prelude::*;
use std::cell::{Cell, RefCell};
use std::time::Instant;

pub fn regular_polygons() -> RegularPolygons {
    RegularPolygons {
        start_time: RefCell::new(None),
        num_drawn: Cell::new(0),
        num_repeats: 1,
        resolution_generator: 3.into(),
        radius_generator: 0.001.into(),
        center_generator: PointGenerator::default(),
        radius_is_constant_for_each_polygon: true,
        has_drawn: Cell::new(false),
        average_duration_of_each_draw_micros: Cell::new(None),
        stroke_weight_generator: 0.001.into(),
        color_generator: soft_black().into(),
    }
}

fn regular_polygon() -> RegularPolygons {
    regular_polygons()
}

pub struct RegularPolygons {
    start_time: RefCell<Option<Instant>>,
    num_drawn: Cell<usize>,
    num_repeats: usize,
    resolution_generator: UsizeGenerator,
    radius_generator: SignalGenerator,
    radius_is_constant_for_each_polygon: bool,
    center_generator: PointGenerator,
    stroke_weight_generator: F32Generator,
    has_drawn: Cell<bool>,
    average_duration_of_each_draw_micros: Cell<Option<f64>>,
    color_generator: ColorGenerator,
}

impl RegularPolygons {
    pub fn repeat(mut self, num_repeats: usize) -> Self {
        self.num_repeats = num_repeats;
        self
    }

    pub fn resolution(mut self, resolution_generator: impl Into<UsizeGenerator>) -> Self {
        self.resolution_generator = resolution_generator.into();
        self
    }

    pub fn radius(mut self, radius_generator: impl Into<SignalGenerator>) -> Self {
        self.radius_generator = radius_generator.into();
        self
    }

    pub fn center(mut self, center_generator: impl Into<PointGenerator>) -> Self {
        self.center_generator = center_generator.into();
        self
    }

    pub fn stroke_weight(mut self, stroke_weight_generator: impl Into<F32Generator>) -> Self {
        self.stroke_weight_generator = stroke_weight_generator.into();
        self
    }

    pub fn radius_is_constant_for_each_polygon(
        mut self,
        radius_is_constant_for_each_polygon: bool,
    ) -> Self {
        self.radius_is_constant_for_each_polygon = radius_is_constant_for_each_polygon;
        self
    }

    pub fn color(mut self, color_generator: impl Into<ColorGenerator>) -> RegularPolygons {
        self.color_generator = color_generator.into();
        self
    }

    pub fn num_to_draw_this_frame(&self) -> usize {
        let num_remaining = self.num_remaining();
        let num_to_draw_per_frame = self.num_to_draw_per_frame();
        num_to_draw_per_frame.min(num_remaining)
    }

    pub fn num_remaining(&self) -> usize {
        self.num_repeats - self.num_drawn.get()
    }

    pub fn num_to_draw_per_frame(&self) -> usize {
        match self.average_duration_of_each_draw_micros.get() {
            None => 1, // If we haven't drawn yet, perform a single draw to see how long it takes.
            Some(average_duration_of_each_draw_micros) => {
                let num_draws_in_target_frame_duration =
                    target_frame_duration_micros / average_duration_of_each_draw_micros;

                let num_to_draw_per_frame = num_draws_in_target_frame_duration.round() as usize;
                num_to_draw_per_frame.clamp(1, 1_000_000)
            }
        }
    }

    pub fn into_drawing(self) -> Drawing {
        Drawing {
            crayon: RefCell::new(Box::new(self)),
        }
    }
}

impl Crayon for RegularPolygons {
    fn draw(&mut self, params: &DrawParams) {
        // Do this instead of a match to make sure that the immutable
        // borrow of self.start_time is dropped before we call replace.
        let has_set_start_time = self.start_time.borrow().is_some();
        if !has_set_start_time {
            self.start_time.replace(Some(Instant::now()));
        }

        self.has_drawn.set(true);

        if params.app.elapsed_frames() == 0 {
            params.draw.background().color(soft_white());
        }

        let draw_start = Instant::now();

        let rand = params.rand_that_changes_every_frame();
        let num_draws_this_frame = self.num_to_draw_this_frame();
        let num_drawn = self.num_drawn.get();
        let max_draw_index = self.num_repeats - 1;
        for current_draw_index in 0..num_draws_this_frame {
            let total_draw_index = current_draw_index + num_drawn;
            let progress_through_whole_drawing = total_draw_index as f32 / max_draw_index as f32;

            let center = self.center_generator.generate(&rand);
            let resolution = self.resolution_generator.generate(&rand);
            let stroke_weight = self.stroke_weight_generator.generate(&rand);

            let path = if self.radius_is_constant_for_each_polygon {
                let radius = self
                    .radius_generator
                    .generate(&rand, progress_through_whole_drawing);
                Path2::regular_polygon(&center, resolution, |_| radius)
            } else {
                Path2::regular_polygon(&center, resolution, |normalized_angle| {
                    self.radius_generator.generate(&rand, normalized_angle)
                })
            };

            let color = self.color_generator.generate(&rand);

            params
                .draw
                .polyline()
                .stroke_weight(stroke_weight)
                .points(path)
                .color(color);

            // std::thread::sleep(std::time::Duration::from_millis(8));
        }

        let total_draw_duration = draw_start.elapsed().as_micros() as f64;
        let average_duration_of_each_draw_micros =
            total_draw_duration / num_draws_this_frame as f64;
        self.average_duration_of_each_draw_micros
            .set(Some(average_duration_of_each_draw_micros));

        self.num_drawn.set(num_drawn + num_draws_this_frame);
    }

    fn update(&mut self) -> DoneRendering {
        // Make sure that draw runs before update.
        let has_drawn = self.has_drawn.get();
        if !has_drawn {
            return DoneRendering::No;
        }

        let num_drawn = self.num_drawn.get();
        if num_drawn >= self.num_repeats {
            let elapsed = self.start_time.borrow().unwrap().elapsed();
            println!("Elapsed: {:?}", elapsed);
            DoneRendering::Yes
        } else {
            DoneRendering::No
        }
    }
}

const micros_in_second: f64 = 1_000_000.0;
const frames_per_second: f64 = 60.0;
const target_frame_duration_micros: f64 = micros_in_second / frames_per_second;
