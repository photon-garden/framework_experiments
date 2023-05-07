use crate::prelude::*;

pub fn render(params: &DrawParams) {
    let mut predictor = ColorPredictor::new(
        params.app,
        params.rand.clone(),
        "ny-yayoi-kusama-blurred.jpg",
    );
    let grid = RectGrid::unit(100, 100);

    for cell in grid.cells.into_iter() {
        let color = predictor.next();

        params
            .draw
            .rect()
            .wh(cell.rect.wh())
            .xy(cell.rect.xy())
            .color(color);
    }
}
