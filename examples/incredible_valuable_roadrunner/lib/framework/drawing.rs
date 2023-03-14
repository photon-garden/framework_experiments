use crate::prelude::*;
use std::cell::RefCell;

pub struct Drawing {
    pub crayon: RefCell<Box<dyn Crayon>>,
}

impl Drawing {
    pub fn draw(&self, params: &DrawParams) {
        self.crayon.borrow_mut().draw(params);
    }

    pub fn update(&mut self) -> DoneRendering {
        self.crayon.borrow_mut().update()
    }
}

pub trait Crayon {
    fn draw(&mut self, params: &DrawParams);

    fn update(&mut self) -> DoneRendering;
}
