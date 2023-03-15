use crate::artworks;
use crate::prelude::*;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::Instant;

pub struct Model {
    pub snapshot: crate::snapshot::Snapshot,
    pub container: DenormalizedRect,
    pub done_rendering: bool,
    pub rand: Rand,
    drawing_mut: RefCell<DrawingMut>,
}

impl Model {
    pub fn new(app: &App) -> Self {
        let snapshot = snapshot::save();
        let rand = snapshot.get_rand();
        let container = get_container_rect();

        let root_params = RootParams {
            app,
            rand: &rand,
            container: &container,
        };

        let artwork = crate::artworks::root::create(&root_params);
        let drawing_mut = DrawingMut::new(artwork);

        Model {
            snapshot,
            container,
            done_rendering: false,
            rand,
            drawing_mut: RefCell::new(drawing_mut),
        }
    }

    pub fn draw(&self, app: &App, frame: Frame) {
        self.drawing_mut
            .borrow_mut()
            .draw_artwork_in_a_loop(app, self, frame);
    }

    pub fn update(&mut self, app: &App) {
        if self.done_rendering {
            return;
        }

        if let DoneDrawing::Yes = self.drawing_mut.borrow_mut().update() {
            println!("Done drawing.");
            self.snapshot.capture_frame(app);
            self.done_rendering = true;
            app.set_loop_mode(LoopMode::loop_once())
        }
    }
}

struct DrawingMut {
    artwork: Box<dyn Artwork>,
    has_drawn: bool,
    start_time: Option<Instant>,
    num_drawn: usize,
    num_repeats: usize,
    average_duration_of_each_draw_micros: Option<f64>,
    background_color: Hsl,
}

impl DrawingMut {
    fn new<A>(artwork: A) -> Self
    where
        A: Artwork + 'static,
    {
        let num_repeats = artwork.num_repeats();

        DrawingMut {
            artwork: Box::new(artwork),
            has_drawn: false,
            start_time: None,
            num_drawn: 0,
            num_repeats,
            average_duration_of_each_draw_micros: None,
            background_color: Hsl::new(0.0, 0.0, 0.0),
        }
    }

    fn draw_artwork_in_a_loop(&mut self, app: &App, model: &Model, frame: Frame) {
        if model.done_rendering {
            return;
        }

        self.start_time.get_or_insert_with(Instant::now);

        self.has_drawn = true;

        let rand = model.snapshot.get_rand();
        let container = &model.container;

        // Scale and translate the draw instance so that we can use normalized points.
        let draw = app
            .draw()
            .scale_axes(vec3(container.w(), container.h(), container.w()))
            .translate(vec3(-0.5, -0.5, -0.5));

        let mut params = DrawParams {
            app,
            model,
            rand: &rand,
            draw: &draw,
            container,
            progress_through_whole_drawing: 0.0,
        };

        if app.elapsed_frames() == 0 {
            draw.background().color(self.background_color);
        }

        let num_draws_this_frame = self.num_draws_this_frame();
        let max_draw_index = self.num_repeats - 1;
        let draw_start = Instant::now();

        for current_draw_index in 0..num_draws_this_frame {
            let total_draw_index = current_draw_index + self.num_drawn;
            params.progress_through_whole_drawing = total_draw_index as f32 / max_draw_index as f32;

            self.artwork.draw(&params);
            // std::thread::sleep(std::time::Duration::from_millis(8));
        }

        let total_draw_duration = draw_start.elapsed().as_micros() as f64;
        let average_duration_of_each_draw_micros =
            total_draw_duration / num_draws_this_frame as f64;
        self.average_duration_of_each_draw_micros = Some(average_duration_of_each_draw_micros);

        self.num_drawn += num_draws_this_frame;

        draw.to_frame(app, &frame).unwrap();
    }

    fn update(&mut self) -> DoneDrawing {
        // Make sure that draw runs before update.
        if !self.has_drawn {
            return DoneDrawing::No;
        }

        if self.num_drawn >= self.num_repeats {
            let elapsed = self.start_time.unwrap().elapsed();
            println!("Done drawing. Time taken: {:?}", elapsed);
            DoneDrawing::Yes
        } else {
            DoneDrawing::No
        }
    }

    pub fn num_draws_this_frame(&self) -> usize {
        let num_draws_remaining = self.num_draws_remaining();
        let num_draws_per_frame = self.num_draws_per_frame();
        num_draws_per_frame.min(num_draws_remaining)
    }

    pub fn num_draws_remaining(&self) -> usize {
        self.num_repeats - self.num_drawn
    }

    pub fn num_draws_per_frame(&self) -> usize {
        match self.average_duration_of_each_draw_micros {
            None => 1, // If we haven't drawn yet, perform a few draws to see how long it takes.
            Some(average_duration_of_each_draw_micros) => {
                let num_draws_in_target_frame_duration =
                    target_frame_duration_micros / average_duration_of_each_draw_micros;

                let num_draws_per_frame = num_draws_in_target_frame_duration.round() as usize;
                num_draws_per_frame.clamp(1, 1_000_000)
            }
        }
    }
}

pub trait Artwork {
    fn draw(&mut self, params: &DrawParams);
    fn num_repeats(&self) -> usize;
}

const micros_in_second: f64 = 1_000_000.0;
const frames_per_second: f64 = 60.0;
const target_frame_duration_micros: f64 = micros_in_second / frames_per_second;
