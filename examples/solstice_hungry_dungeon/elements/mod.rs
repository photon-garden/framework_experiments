use crate::prelude::*;

pub mod background;

mod box_shadow;

pub mod color_swatch;
pub use color_swatch::*;

mod color_palette;
mod color_predictor;

pub mod element;
pub use element::*;

pub use root::*;
pub mod root;

pub mod shell_bundle_test;

pub mod words;
pub use words::*;

pub struct DrawParams<'a> {
    pub app: &'a App,
    pub rand: &'a Rand,
    pub draw: &'a Draw,
    pub model: &'a Model,
    pub container: &'a Rect,
}

impl<'a> DrawParams<'a> {
    pub fn rand_that_stays_the_same_every_frame(&self) -> &'a Rand {
        self.rand
    }

    pub fn rand_that_changes_every_frame(&self) -> &'a Rand {
        &self.model.rand
    }
}

pub struct UpdateParams<'a> {
    pub app: &'a App,
    pub update: Update,
    pub rand: &'a Rand,
}

pub enum DoneRendering {
    Yes,
    No,
}

impl DoneRendering {
    pub fn all_done(values: Vec<DoneRendering>) -> DoneRendering {
        let all_done = values.iter().all(|value| value.to_bool());
        all_done.into()
    }

    pub fn to_bool(&self) -> bool {
        match self {
            DoneRendering::Yes => true,
            DoneRendering::No => false,
        }
    }
}

impl From<bool> for DoneRendering {
    fn from(is_done: bool) -> Self {
        if is_done {
            DoneRendering::Yes
        } else {
            DoneRendering::No
        }
    }
}

pub trait UpdateDraw {
    fn update(&mut self, _params: &UpdateParams) -> DoneRendering {
        DoneRendering::No
    }

    fn draw(&self, params: &DrawParams);
}
