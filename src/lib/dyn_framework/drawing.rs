use crate::prelude::*;
use std::cell::{Cell, RefCell};
use std::time::Instant;

pub struct Drawing {
    pub inner: RefCell<DrawingMut>,
}

impl Drawing {
    pub fn draw(&self, params: &DrawParams) {
        self.inner.borrow_mut().draw(params);
    }

    pub fn update(&mut self) -> DoneDrawing {
        self.inner.borrow_mut().update()
    }
}

struct DrawingMut {
    artwork: Box<dyn Artwork>,
    has_drawn: Cell<bool>,
    start_time: RefCell<Option<Instant>>,
    num_drawn: Cell<usize>,
    num_repeats: usize,
    average_duration_of_each_draw_micros: Cell<Option<f64>>,
    background_color: Hsl,
}

impl DrawingMut {
    fn draw(&mut self, params: &DrawParams) {
        // Do this instead of a match to make sure that the immutable
        // borrow of self.start_time is dropped before we call replace.
        //
        // Pretty sure there's a more elegant method on Option to do this.
        let has_set_start_time = self.start_time.borrow().is_some();
        if !has_set_start_time {
            self.start_time.replace(Some(Instant::now()));
        }

        self.has_drawn.set(true);

        if params.app.elapsed_frames() == 0 {
            params.draw.background().color(self.background_color);
        }

        let draw_start = Instant::now();
        let rand = params.rand_that_changes_every_frame();
        let num_draws_this_frame = self.num_draws_this_frame();
        let num_drawn = self.num_drawn.get();
        let max_draw_index = self.num_repeats - 1;

        for current_draw_index in 0..num_draws_this_frame {
            let total_draw_index = current_draw_index + num_drawn;
            let progress_through_whole_drawing = total_draw_index as f32 / max_draw_index as f32;

            self.artwork.draw(params);
            // std::thread::sleep(std::time::Duration::from_millis(8));
        }

        let total_draw_duration = draw_start.elapsed().as_micros() as f64;
        let average_duration_of_each_draw_micros =
            total_draw_duration / num_draws_this_frame as f64;
        self.average_duration_of_each_draw_micros
            .set(Some(average_duration_of_each_draw_micros));

        self.num_drawn.set(num_drawn + num_draws_this_frame);
    }

    fn update(&mut self) -> DoneDrawing {
        // Make sure that draw runs before update.
        let has_drawn = self.has_drawn.get();
        if !has_drawn {
            return DoneDrawing::No;
        }

        let num_drawn = self.num_drawn.get();
        if num_drawn >= self.num_repeats {
            let elapsed = self.start_time.borrow().unwrap().elapsed();
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
        self.num_repeats - self.num_drawn.get()
    }

    pub fn num_draws_per_frame(&self) -> usize {
        match self.average_duration_of_each_draw_micros.get() {
            None => 3, // If we haven't drawn yet, perform a few draws to see how long it takes.
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
}

const micros_in_second: f64 = 1_000_000.0;
const frames_per_second: f64 = 60.0;
const target_frame_duration_micros: f64 = micros_in_second / frames_per_second;
