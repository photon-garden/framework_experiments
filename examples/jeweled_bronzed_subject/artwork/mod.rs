use crate::prelude::*;

mod box_shadow;

pub mod color_swatch;
pub use color_swatch::*;

mod color_palette;
mod color_predictor;

pub use root::*;
pub mod root;

mod regular_polygons;
pub use regular_polygons::*;

pub mod words;
pub use words::*;

pub trait Artwork {
    fn draw(&mut self, params: &DrawParams);
    fn num_repeats(&self) -> usize;
    fn background_color(&self) -> Hsl;
}

pub struct DrawParams<'a> {
    pub app: &'a App,
    pub rand: &'a Rand,
    pub draw: &'a Draw,
    pub model: &'a Model,
    pub container: &'a Rect,
    pub progress_through_whole_drawing: NormalizedF32,
}

impl<'a> DrawParams<'a> {
    pub fn rand_that_stays_the_same_every_frame(&self) -> &'a Rand {
        self.rand
    }

    pub fn rand_that_changes_every_frame(&self) -> &'a Rand {
        &self.model.rand
    }
}

pub enum DoneDrawing {
    Yes,
    No,
}

impl DoneDrawing {
    pub fn all_done(values: Vec<DoneDrawing>) -> DoneDrawing {
        let all_done = values.iter().all(|value| value.to_bool());
        all_done.into()
    }

    pub fn to_bool(&self) -> bool {
        match self {
            DoneDrawing::Yes => true,
            DoneDrawing::No => false,
        }
    }
}

impl From<bool> for DoneDrawing {
    fn from(is_done: bool) -> Self {
        if is_done {
            DoneDrawing::Yes
        } else {
            DoneDrawing::No
        }
    }
}
