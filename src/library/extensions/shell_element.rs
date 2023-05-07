pub use crate::prelude::*;

pub type ShellElement = [Point2; 2];

pub trait ShellElementExtension {
    fn midpoint(&self) -> Point2;
    fn lerp(&self, progress: f32, other: &ShellElement) -> ShellElement;
    fn distance(&self, other: &ShellElement) -> f32;
}

impl ShellElementExtension for ShellElement {
    fn midpoint(&self) -> Point2 {
        let [left, right] = self;
        left.lerp(0.5, right)
    }

    fn lerp(&self, progress: f32, other: &ShellElement) -> ShellElement {
        let [left, right] = self;
        let [other_left, other_right] = other;

        let new_left = left.lerp(progress, other_left);
        let new_right = right.lerp(progress, other_right);

        [new_left, new_right]
    }

    fn distance(&self, other: &ShellElement) -> f32 {
        let midpoint = self.midpoint();
        let other_midpoint = other.midpoint();
        midpoint.distance(other_midpoint)
    }
}
