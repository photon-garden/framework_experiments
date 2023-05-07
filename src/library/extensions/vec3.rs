use crate::prelude::*;

pub trait Vec3Extension {
    fn from_angle(x_turns: NumberOfTurns, y_turns: NumberOfTurns) -> Vec3 {
        let alpha = y_turns.turns_to_radians();
        let beta = x_turns.turns_to_radians();

        let x = alpha.cos() * beta.cos();
        let y = beta.sin();
        let z = alpha.sin() * beta.cos();

        vec3(x, y, z)
    }

    fn between(self, other: Vec3) -> Vec3;

    fn signed_angle_between(&self, other: Vec3, perpendicular_to_plane: Vec3) -> f32;
}

impl Vec3Extension for Vec3 {
    fn between(self, other: Vec3) -> Vec3 {
        other - self
    }

    // This assumes self and other lie in the same plane and have the same origin.
    // perpendicular_to_plane is the vector that's normal (perpendicular) to the plane.
    //
    // https://stackoverflow.com/questions/5188561/signed-angle-between-two-3d-vectors-with-same-origin-within-the-same-plane
    //
    // atan2((Va x Vb) . Vn, Va . Vb)
    fn signed_angle_between(&self, other: Vec3, perpendicular_to_plane: Vec3) -> f32 {
        let normalized_perpendicular_to_plane = perpendicular_to_plane.try_normalize().unwrap();
        let a = self.cross(other).dot(normalized_perpendicular_to_plane);
        let b = self.dot(other);
        a.atan2(b)
    }
}
