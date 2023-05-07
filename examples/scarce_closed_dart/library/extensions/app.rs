use nannou::prelude::*;

use super::rect::RectExtension;

pub trait AppExtension {
    fn normalized_mouse_x(&self) -> f32;
    fn normalized_mouse_y(&self) -> f32;
    fn normalized_mouse_xy(&self) -> Point2;
}

impl AppExtension for App {
    fn normalized_mouse_x(&self) -> f32 {
        let mouse_x = self.mouse.x;
        self.main_window().rect().normalize_w(mouse_x)
    }
    fn normalized_mouse_y(&self) -> f32 {
        let mouse_y = self.mouse.y;
        self.main_window().rect().normalize_h(mouse_y)
    }
    fn normalized_mouse_xy(&self) -> Point2 {
        let mouse_x = self.normalized_mouse_x();
        let mouse_y = self.normalized_mouse_y();

        pt2(mouse_x, mouse_y)
    }
}
