use crate::prelude::*;
use std::iter;

pub fn new(params: &DrawParams) {
    let draw = params.app.draw();
    let container = params.container;
    let text_container = container.scale(0.8);
    let text_container_wh = text_container.wh();
    let font_size = text_container_wh.x.times(0.03) as u32;

    let font_path = params
        .app
        .assets_path()
        .unwrap()
        .join("Inter")
        .join("static")
        .join("Inter-Thin.ttf");
    let font = text::font::from_file(font_path).unwrap();
    let text = open_ai::create_text_completion("Write a short haiku in the style of Matsuo Basho.");
    draw.text(&text)
        .color(soft_black())
        .font_size(font_size)
        .font(font)
        .wh(text_container_wh);
}
