use crate::elements;
use crate::prelude::*;
use std::rc::Rc;

pub struct Model {
    pub snapshot: crate::snapshot::Snapshot,
    pub container: DenormalizedRect,
    pub done_rendering: bool,
    pub rand: Rand,
    pub root_element: Root,
    // pub drawing: crate::framework::Drawing,
    pub drawing: crate::lib::dyn_framework::Drawing,
}

impl Model {
    pub fn new(app: &App) -> Self {
        let snapshot = snapshot::save();
        let rand = snapshot.get_rand();
        let container = get_container_rect();

        let root_element = Root::new(&RootParams {
            app,
            rand: &rand,
            container: &container,
        });

        Model {
            snapshot,
            container,
            done_rendering: false,
            rand,
            root_element,
            // drawing: crate::framework::draw().into_drawing(),
            drawing: crate::lib::dyn_framework::draw().into_drawing(),
        }
    }
}
