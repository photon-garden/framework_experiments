use crate::prelude::*;
use std::cell::RefCell;

pub struct Drawing {
    pub drawing_mut: RefCell<Box<dyn DrawingMut>>,
}

impl Drawing {
    pub fn draw(&self, params: &DrawParams) {
        self.drawing_mut.borrow_mut().draw(params);
    }

    pub fn update(&mut self) -> DoneRendering {
        self.drawing_mut.borrow_mut().update()
    }
}

pub trait DrawingMut {
    fn draw(&mut self, params: &DrawParams);
    fn update(&mut self) -> DoneRendering;
}
