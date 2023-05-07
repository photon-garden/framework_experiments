use crate::prelude::*;

pub type NormalizedVec2 = Vec2;

pub trait Vec2Extension {
    fn from_angle(angle: NumberOfTurns) -> Vec2;
    fn between(a: &Vec2, b: &Vec2) -> Vec2;
    fn perpendicular_clockwise(&self) -> Vec2;
    fn perpendicular_counterclockwise(&self) -> Vec2;
    fn resize(&self, desired_length: f32) -> Vec2;
    fn divided_by(&self, denominator: f32) -> Vec2;
    fn normalized_angle(&self) -> NumberOfTurns;
}

impl Vec2Extension for Vec2 {
    fn from_angle(angle: NumberOfTurns) -> Vec2 {
        let theta = angle.turns_to_radians();

        let x = theta.cos();
        let y = theta.sin();

        vec2(x, y)
    }
    fn between(a: &Vec2, b: &Vec2) -> Vec2 {
        *b - *a
    }
    fn perpendicular_clockwise(&self) -> Vec2 {
        vec2(self.y, -self.x)
    }
    fn perpendicular_counterclockwise(&self) -> Vec2 {
        vec2(-self.y, self.x)
    }
    // https://math.stackexchange.com/questions/897723/how-to-resize-a-vector-to-a-specific-length
    fn resize(&self, desired_length: f32) -> Vec2 {
        let original_length = self.length();
        let ratio = desired_length / original_length;

        vec2(ratio * self.x, ratio * self.y)
    }
    fn divided_by(&self, denominator: f32) -> Vec2 {
        vec2(self.x / denominator, self.y / denominator)
    }
    // The angle() method returns radians. Use this function if you
    // want an angle in the range 0 to 1 instead.
    fn normalized_angle(&self) -> NumberOfTurns {
        self.angle().divided_by(TAU)
    }
}
