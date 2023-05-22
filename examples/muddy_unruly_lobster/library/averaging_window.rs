use crate::prelude::*;
use std::collections::VecDeque;
use std::ops::{Add, Div};

pub struct AveragingWindow<Element>
where
    Element: CanBeAveraged,
{
    size: usize,
    elements: VecDeque<Element>,
}

impl<Element> AveragingWindow<Element>
where
    Element: CanBeAveraged,
{
    pub fn new(size: usize) -> AveragingWindow<Element> {
        let elements = VecDeque::with_capacity(size);
        AveragingWindow { size, elements }
    }

    pub fn add(&mut self, element: Element) {
        self.elements.push_back(element);

        if self.elements.len() > self.size {
            self.elements.pop_front();
        }
    }

    pub fn average(&self) -> Element {
        let mut maybe_total_so_far: Option<Element> = None;

        for element in self.elements.iter() {
            let new_total = match maybe_total_so_far {
                Some(total_so_far) => total_so_far + *element,
                None => *element,
            };

            maybe_total_so_far = Some(new_total);
        }

        let total = maybe_total_so_far.expect("Tried to get the average from an AveragingWindow, but the window didn't have any elements.");
        let len = self.size as f32;

        total / len
    }
}

pub trait CanBeAveraged: Add<Self, Output = Self> + Div<f32, Output = Self> + Sized + Copy {}

impl CanBeAveraged for Vec3 {}
impl CanBeAveraged for Point2 {}
impl CanBeAveraged for f32 {}
