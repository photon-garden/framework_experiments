use crate::prelude::*;
use nannou::color::Hue;
use nannou::color::Saturate;
use nannou::color::Shade;
use spade::{DelaunayTriangulation, Triangulation};

pub fn new(_params: &RootParams) -> Element {
    Element::once(|params| {
        params.draw.background().color(soft_white());
    })
}
