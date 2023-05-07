use crate::prelude::*;

const unscaled_window_width: f32 = 1000.0;
const unscaled_window_height: f32 = 1000.0;
pub const window_scale: f32 = 1.0;

pub const container_scale: f32 = 0.9;

pub fn get_window_dimensions() -> [f32; 2] {
    let window_width = unscaled_window_width.times(window_scale).round();
    let window_height = unscaled_window_height.times(window_scale).round();

    [window_width, window_height]
}

pub fn get_container_rect() -> DenormalizedRect {
    let [window_width, window_height] = get_window_dimensions();

    let naive_container_width = window_width.times(container_scale);
    let naive_container_height = window_height.times(container_scale);

    let x_border = window_width - naive_container_width;
    let y_border = window_height - naive_container_height;

    let border = x_border.min(y_border);

    let container_width = window_width - border;
    let container_height = window_height - border;

    let container_wh = vec2(container_width, container_height);

    let center = pt2(0.0, 0.0);

    Rect::from_xy_wh(center, container_wh)
}

pub fn aspect_ratio() -> f32 {
    let [window_width, window_height] = get_window_dimensions();
    window_width / window_height
}
