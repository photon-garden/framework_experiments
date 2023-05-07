use std::ops::RangeInclusive;

use crate::prelude::*;

pub type NormalizedRect = Rect;
pub type DenormalizedRect = Rect;

pub trait RectExtension {
    fn unit() -> Rect;
    fn minus_one_to_one() -> Rect;
    fn left_right(&self) -> RangeInclusive<f32>;
    fn bottom_top(&self) -> RangeInclusive<f32>;
    fn normalized_pad_w(&self, padding: f32) -> Rect;
    fn normalize_rect(&self, other: &DenormalizedRect) -> Rect;
    fn normalize_xy(&self, point: &DenormalizedPoint2) -> NormalizedPoint2;
    fn pad_w(&self, padding: f32) -> Rect;
    fn normalized_pad_h(&self, padding: f32) -> Rect;
    fn pad_h(&self, padding: f32) -> Rect;
    fn scale(&self, amount: f32) -> Rect;
    fn scale_w(&self, amount: f32) -> Rect;
    fn scale_h(&self, amount: f32) -> Rect;
    fn scale_w_h(&self, w_amount: f32, h_amount: f32) -> Rect;
    fn lerp_w(&self, x: f32) -> f32;
    fn lerp_h(&self, x: f32) -> f32;
    fn lerp_wh(&self, point: &Vec2) -> Vec2;
    fn denormalize_x(&self, x: f32) -> f32;
    fn denormalize_y(&self, y: f32) -> f32;
    fn denormalize_xy(&self, point: &Point2) -> Point2;
    fn denormalize_x_y(&self, x: f32, y: f32) -> Point2;
    fn denormalize_rect(&self, rect: &NormalizedRect) -> DenormalizedRect;
    fn width_denormalize_xy(&self, point: &Point2) -> Point2;
    fn height_denormalize_xy(&self, point: &Point2) -> Point2;
    fn width_denormalize_x_y(&self, x: f32, y: f32) -> Point2;
    fn height_denormalize_x_y(&self, x: f32, y: f32) -> Point2;
    fn normalize_w(&self, x: f32) -> f32;
    fn normalize_h(&self, y: f32) -> f32;
    fn normalize_wh(&self, point: &Point2) -> Point2;
    fn grid(&self, num_columns: usize, num_rows: usize) -> RectGrid;
    fn child(
        &self,
        top_left_x: f32,
        top_left_y: f32,
        bottom_right_x: f32,
        bottom_right_y: f32,
    ) -> Rect;
    fn point_within(&self, normalized_x: f32, normalized_y: f32) -> Point2;
    fn random_point_within(&self, rand: &Rand) -> Point2;
    fn smallest_dimension(&self) -> f32;
    fn largest_centered_child(&self) -> Rect;
    fn as_shell(&self, resolution: usize) -> Shell;
}

impl RectExtension for Rect {
    fn unit() -> Rect {
        Rect::from_corners(pt2(0.0, 0.0), pt2(1.0, 1.0))
    }

    fn minus_one_to_one() -> Rect {
        Rect::from_corners(pt2(-1.0, -1.0), pt2(1.0, 1.0))
    }

    fn left_right(&self) -> RangeInclusive<f32> {
        self.left()..=self.right()
    }

    fn bottom_top(&self) -> RangeInclusive<f32> {
        self.bottom()..=self.top()
    }

    fn normalized_pad_w(&self, normalized_padding: f32) -> Rect {
        let padding = self.lerp_w(normalized_padding);
        self.pad_w(padding)
    }

    fn normalize_rect(&self, other: &DenormalizedRect) -> Rect {
        let top_left = self.normalize_xy(&other.top_left());
        let bottom_right = self.normalize_xy(&other.bottom_right());

        Rect::from_corners(top_left, bottom_right)
    }

    fn normalize_xy(&self, point: &DenormalizedPoint2) -> NormalizedPoint2 {
        let x = point.x.normalize(self.left(), self.right());
        let y = point.y.normalize(self.bottom(), self.top());
        pt2(x, y)
    }

    fn pad_w(&self, padding: f32) -> Rect {
        self.pad_left(padding).pad_right(padding)
    }

    fn normalized_pad_h(&self, normalized_padding: f32) -> Rect {
        let padding = self.lerp_h(normalized_padding);
        self.pad_h(padding)
    }

    fn pad_h(&self, padding: f32) -> Rect {
        self.pad_top(padding).pad_bottom(padding)
    }

    fn scale(&self, amount: f32) -> Rect {
        self.scale_w_h(amount, amount)
    }

    fn scale_w(&self, amount: f32) -> Rect {
        let new_w = self.w() * amount;
        Rect::from_x_y_w_h(self.x(), self.y(), new_w, self.h())
    }

    fn scale_h(&self, amount: f32) -> Rect {
        let new_h = self.h() * amount;
        Rect::from_x_y_w_h(self.x(), self.y(), self.w(), new_h)
    }

    fn scale_w_h(&self, w_amount: f32, h_amount: f32) -> Rect {
        let new_w = self.w() * w_amount;
        let new_h = self.h() * h_amount;
        Rect::from_x_y_w_h(self.x(), self.y(), new_w, new_h)
    }

    fn lerp_w(&self, x: f32) -> f32 {
        x * self.w()
    }

    fn lerp_h(&self, y: f32) -> f32 {
        y * self.h()
    }

    fn lerp_wh(&self, point: &Vec2) -> Vec2 {
        pt2(self.lerp_w(point.x), self.lerp_h(point.y))
    }

    fn denormalize_xy(&self, point: &Point2) -> Point2 {
        self.denormalize_x_y(point.x, point.y)
    }

    fn denormalize_x(&self, x: f32) -> f32 {
        x.denormalize(self.left(), self.right())
    }

    fn denormalize_y(&self, y: f32) -> f32 {
        y.denormalize(self.bottom(), self.top())
    }

    fn denormalize_rect(&self, rect: &NormalizedRect) -> DenormalizedRect {
        let top_left = self.denormalize_xy(&rect.top_left());
        let bottom_right = self.denormalize_xy(&rect.bottom_right());

        Rect::from_corners(top_left, bottom_right)
    }

    fn denormalize_x_y(&self, x: f32, y: f32) -> Point2 {
        pt2(self.denormalize_x(x), self.denormalize_y(y))
    }

    fn width_denormalize_xy(&self, point: &Point2) -> Point2 {
        self.width_denormalize_x_y(point.x, point.y)
    }

    fn height_denormalize_xy(&self, point: &Point2) -> Point2 {
        self.height_denormalize_x_y(point.x, point.y)
    }

    fn width_denormalize_x_y(&self, x: f32, y: f32) -> Point2 {
        pt2(self.denormalize_x(x), self.denormalize_x(y))
    }

    fn height_denormalize_x_y(&self, x: f32, y: f32) -> Point2 {
        pt2(self.denormalize_y(x), self.denormalize_y(y))
    }

    fn normalize_w(&self, x: f32) -> f32 {
        x.normalize(self.left(), self.right())
    }

    fn normalize_h(&self, y: f32) -> f32 {
        y.normalize(self.bottom(), self.top())
    }

    fn normalize_wh(&self, point: &Point2) -> Point2 {
        pt2(self.normalize_w(point.x), self.normalize_h(point.y))
    }

    fn grid(&self, num_columns: usize, num_rows: usize) -> RectGrid {
        RectGrid::new(*self, num_columns, num_rows)
    }

    fn child(
        &self,
        top_left_x: f32,
        top_left_y: f32,
        bottom_right_x: f32,
        bottom_right_y: f32,
    ) -> Rect {
        let top_left = self.point_within(top_left_x, top_left_y);
        let bottom_right = self.point_within(bottom_right_x, bottom_right_y);

        Rect::from_corners(top_left, bottom_right)
    }

    fn point_within(&self, normalized_x: f32, normalized_y: f32) -> Point2 {
        let new_x = normalized_x.denormalize(self.left(), self.right());
        let new_y = normalized_y.denormalize(self.bottom(), self.top());

        pt2(new_x, new_y)
    }

    fn random_point_within(&self, rand: &Rand) -> Point2 {
        let x = rand.zero_to_one();
        let y = rand.zero_to_one();

        self.point_within(x, y)
    }

    fn smallest_dimension(&self) -> f32 {
        let w = self.w();
        let h = self.h();

        w.min(h)
    }

    // Gets the largest rectangle that can fit inside this one, centered within this rectangle.
    fn largest_centered_child(&self) -> Rect {
        let smallest_dimension = self.smallest_dimension();
        let wh = vec2(smallest_dimension, smallest_dimension);
        Rect::from_xy_wh(self.xy(), wh)
    }

    fn as_shell(&self, resolution: usize) -> Shell {
        if resolution < 2 {
            panic!("Resolution has to be at least two.");
        }

        let top_left = self.top_left();
        let top_right = self.top_right();
        let bottom_left = self.bottom_left();
        let bottom_right = self.bottom_right();

        zero_to_one(resolution)
            .map(|progress| {
                let left = top_left.lerp(bottom_left, progress);
                let right = top_right.lerp(bottom_right, progress);
                [left, right]
            })
            .collect()
    }
}
