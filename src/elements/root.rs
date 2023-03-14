use crate::prelude::*;

pub struct Root {
    elements: Elements,
}

impl Root {
    pub fn new(params: &RootParams) -> Self {
        Root {
            elements: Elements::new(vec![
                //
                background::new(params),
                // tree::new(pt2(0.5, 0.0), vec2(1.0, 1.0)),
                shell_bundle_test::new(),
                // tree::new(pt2(0.06, 0.02), vec2(0.8, 1.0)),
                // tree::new(pt2(0.85, 0.018), vec2(0.82, 0.92)),
                // tree::new(pt2(0.3, 0.00), vec2(0.95, 1.2)),
                // tree::new(pt2(0.63, -0.01), vec2(1.1, 1.3)),
                // color_swatch::new(),
                // words::new(),
            ]),
        }
    }
}

impl UpdateDraw for Root {
    fn draw(&self, params: &DrawParams) {
        self.elements.draw(params);
    }

    fn update(&mut self, _params: &UpdateParams) -> DoneDrawing {
        self.elements.update()
        // DoneDrawing::No
    }
}

pub struct RootParams<'a> {
    pub app: &'a App,
    pub rand: &'a Rand,
    pub container: &'a Rect,
}

struct Elements {
    current_element: Element,
    remaining_elements: Vec<Element>,
}

impl Elements {
    fn new(mut remaining_elements: Vec<Element>) -> Elements {
        let current_element = remaining_elements.remove(0);
        Elements {
            current_element,
            remaining_elements,
        }
    }

    fn update(&mut self) -> DoneDrawing {
        let current_element_done_rendering = self.current_element.update().to_bool();
        if current_element_done_rendering && self.remaining_elements.is_empty() {
            DoneDrawing::Yes
        } else if current_element_done_rendering {
            self.current_element = self.remaining_elements.remove(0);
            DoneDrawing::No
        } else {
            DoneDrawing::No
        }
    }

    fn draw(&self, params: &DrawParams) {
        self.current_element.draw(params);
    }
}
