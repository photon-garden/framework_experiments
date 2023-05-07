use super::*;
use crate::prelude::*;

mod one_point_generator;
pub use one_point_generator::*;

mod uniform_random_point_generator;
pub use uniform_random_point_generator::*;

mod grid_point_generator;
pub use grid_point_generator::*;

mod jitter_point_generator;
pub use jitter_point_generator::*;

pub type Point2Generator = Box<dyn Generator<Output = Point2>>;

pub trait IntoPointGenerator {
    fn into_point_generator(self) -> Box<dyn Generator<Output = Point2>>;
}

impl<G> IntoPointGenerator for G
where
    G: Generator<Output = Point2> + 'static,
{
    fn into_point_generator(self) -> Box<dyn Generator<Output = Point2>> {
        Box::new(self)
    }
}
