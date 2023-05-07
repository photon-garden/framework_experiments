use crate::prelude::*;

pub fn render(colors: Vec<Hsla>, params: &DrawParams) {
    let draw = params.draw;

    let grid = RectGrid::unit(3, 3);
    draw.background().color(WHITE);

    for (index, cell) in grid.cells.into_iter().rev().enumerate() {
        let color = *colors.looped_get(index);

        draw.rect()
            .wh(cell.rect.wh())
            .xy(cell.rect.xy())
            .color(color);
    }
}
