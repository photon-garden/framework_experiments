use crate::prelude::*;

pub type NormalizedPoint3 = Point3;
pub type DenormalizedPoint3 = Point3;

pub trait Point3Extension {
    fn normalized_center() -> Point3;
    fn plus(&self, other: &Point3) -> Point3;
    fn plus_x(&self, amount: f32) -> Point3;
    fn plus_y(&self, amount: f32) -> Point3;
    fn plus_z(&self, amount: f32) -> Point3;
    fn times(&self, multiple: f32) -> Point3;
    fn xy(&self) -> Point2;
    fn xz(&self) -> Point2;
    fn denormalize(&self, min: f32, max: f32) -> Point3;
    fn multi_lerp(&self, other: Point3, resolution: usize) -> Vec<Point3>;
    fn normalize_min_max(&self, min: f32, max: f32) -> Point3;
    fn vector_towards(&self, other: Point3) -> Vec3;
}

impl Point3Extension for Point3 {
    fn normalized_center() -> Point3 {
        pt3(0.5, 0.5, 0.5)
    }

    fn plus(&self, other: &Point3) -> Point3 {
        pt3(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    fn plus_x(&self, amount: f32) -> Point3 {
        pt3(self.x + amount, self.y, self.z)
    }

    fn plus_y(&self, amount: f32) -> Point3 {
        pt3(self.x, self.y + amount, self.z)
    }

    fn plus_z(&self, amount: f32) -> Point3 {
        pt3(self.x, self.y, self.z + amount)
    }

    fn times(&self, multiple: f32) -> Point3 {
        pt3(self.x * multiple, self.y * multiple, self.z * multiple)
    }

    fn xy(&self) -> Point2 {
        pt2(self.x, self.y)
    }

    fn xz(&self) -> Point2 {
        pt2(self.x, self.z)
    }

    fn denormalize(&self, min: f32, max: f32) -> Point3 {
        pt3(
            self.x.denormalize(min, max),
            self.y.denormalize(min, max),
            self.z.denormalize(min, max),
        )
    }

    fn normalize_min_max(&self, min: f32, max: f32) -> Point3 {
        pt3(
            self.x.normalize(min, max),
            self.y.normalize(min, max),
            self.z.normalize(min, max),
        )
    }

    fn multi_lerp(&self, other: Point3, resolution: usize) -> Vec<Point3> {
        zero_to_one(resolution)
            .map(|progress| self.lerp(other, progress))
            .collect()
    }

    fn vector_towards(&self, other: Point3) -> Vec3 {
        other - *self
    }
}
