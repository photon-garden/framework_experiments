use crate::artwork;
use crate::prelude::*;
use std::cell::RefCell;

pub struct Model {
    pub snapshot: crate::snapshot::Snapshot,
    pub container: DenormalizedRect,
    pub done_rendering: bool,
    pub rand: Rand,
    loop_drawer: RefCell<LoopDrawer>,
}

impl Model {
    pub fn new(app: &App) -> Self {
        let snapshot = snapshot::save();
        let rand = snapshot.get_rand();
        let container = get_container_rect();

        let root_params = CreateArtworkParams {
            app,
            rand: &rand,
            container: &container,
        };

        let artwork = artwork::create(root_params);
        let loop_drawer = LoopDrawer::new(artwork);

        Model {
            snapshot,
            container,
            done_rendering: false,
            rand,
            loop_drawer: RefCell::new(loop_drawer),
        }
    }

    pub fn draw(&self, app: &App, frame: Frame) {
        self.loop_drawer
            .borrow_mut()
            .draw_artwork_in_a_loop(app, self, frame);
    }

    pub fn update(&mut self, app: &App) {
        if self.done_rendering {
            return;
        }

        if let DoneDrawing::Yes = self.loop_drawer.borrow_mut().update() {
            self.snapshot.capture_frame(app);
            self.done_rendering = true;
            app.set_loop_mode(LoopMode::loop_once())
        }
    }
}
