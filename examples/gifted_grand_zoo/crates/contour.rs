use nannou::prelude::*;

use crate::extensions::f32::*;

pub type ContourRing = Vec<Point2>;

pub fn rings(
    threshold: f32,
    num_rows: u16,
    num_columns: u16,
    heights: &Vec<f64>,
    container: &Rect,
) -> Vec<ContourRing> {
    let raw_contour_rings = contour::contour_rings(
        heights.as_slice(),
        threshold.into(),
        num_columns.into(),
        num_rows.into(),
    );

    let contour_rings: Vec<ContourRing> = raw_contour_rings
        .iter()
        .map(|raw_contour_ring| {
            to_nannou_contour_ring(num_rows, num_columns, container, raw_contour_ring)
        })
        .collect();

    contour_rings
}

fn to_nannou_contour_ring(
    num_rows: u16,
    num_columns: u16,
    container: &Rect,
    raw_contour_ring: &[Vec<f64>],
) -> ContourRing {
    let contour_ring: ContourRing = raw_contour_ring
        .iter()
        .map(move |coordinates| {
            let x = coordinates[0] as f32;
            let y = coordinates[1] as f32;

            let scaled_x = x.rescale(0.0, num_columns.into(), container.left(), container.right());
            let scaled_y = y.rescale(0.0, num_rows.into(), container.bottom(), container.top());

            pt2(scaled_x, scaled_y)
        })
        .collect();

    contour_ring
}
