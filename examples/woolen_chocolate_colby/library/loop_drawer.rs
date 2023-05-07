use crate::prelude::*;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::time::Instant;

pub static global_draw_index: Lazy<Mutex<usize>> = Lazy::new(|| Mutex::new(0));

pub struct LoopDrawer {
    artwork: Box<dyn Artwork>,
    has_drawn: bool,
    start_time: Option<Instant>,
    num_drawn: usize,
    num_repeats: usize,
    average_duration_of_each_draw_micros: Option<f64>,
    background_color: Hsl,
}

impl LoopDrawer {
    pub fn new<A>(artwork: A) -> Self
    where
        A: Artwork + 'static,
    {
        let num_repeats = artwork.num_repeats();
        let background_color = artwork.background_color();

        LoopDrawer {
            artwork: Box::new(artwork),
            has_drawn: false,
            start_time: None,
            num_drawn: 0,
            num_repeats,
            average_duration_of_each_draw_micros: None,
            background_color,
        }
    }

    pub fn draw_artwork_in_a_loop(&mut self, app: &App, model: &Model, frame: Frame) {
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
            *global_draw_index.lock().unwrap() = current_draw_index;

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

    pub fn update(&mut self) -> DoneDrawing {
        // Make sure that draw runs before update.
        if !self.has_drawn {
            return DoneDrawing::No;
        }

        if self.num_drawn >= self.num_repeats {
            let elapsed = self.start_time.unwrap().elapsed();
            println!("Done drawing. Time taken: {:?}.", elapsed);
            DoneDrawing::Yes
        } else {
            DoneDrawing::No
        }
    }

    fn num_draws_this_frame(&self) -> usize {
        let num_draws_remaining = self.num_draws_remaining();
        let num_draws_per_frame = self.num_draws_per_frame();
        num_draws_per_frame.min(num_draws_remaining)
    }

    fn num_draws_remaining(&self) -> usize {
        self.num_repeats - self.num_drawn
    }

    fn num_draws_per_frame(&self) -> usize {
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

const micros_in_second: f64 = 1_000_000.0;
const frames_per_second: f64 = 60.0;
const target_frame_duration_micros: f64 = micros_in_second / frames_per_second;
