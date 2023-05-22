use crate::prelude::*;
use std::collections::HashMap;
use std::fmt;

type Height = f32;

pub struct Heightmap {
    pub domain: Grid,
    points: HashMap<UsizePoint2, Height>,
}

impl Heightmap {
    pub fn new<GetHeight>(resolution: usize, get_height: GetHeight) -> Heightmap
    where
        GetHeight: Fn(&Point2) -> NormalizedF32,
    {
        let domain = Grid::new(resolution, resolution);
        let mut points = HashMap::new();

        for point in domain.iterate_points() {
            let normalized_point = domain.normalize(&point);
            let height = get_height(&normalized_point);
            points.insert(point, height);
        }

        Heightmap { domain, points }
    }

    pub fn iterate_points(&self) -> impl Iterator<Item = Point3> + '_ {
        self.domain
            .iterate_points()
            .map(|point2| self.normalize(&point2))
    }

    pub fn x_resolution(&self) -> usize {
        self.domain.width
    }

    pub fn z_resolution(&self) -> usize {
        self.domain.height
    }

    // Previous implementation.
    //
    // pub fn surface_normal_at(&self, point: &UsizePoint2) -> Vec3 {
    //     let x = point.x;
    //     let y = point.y;

    //     // For north, south, east, and west.
    //     let adjacent_scale = vec3(0.15, 0.15, 0.15);
    //     let get_adjacent_contribution =
    //         |calculate_x: bool,
    //          first_x_distance: isize,
    //          first_y_distance: isize,
    //          second_x_distance: isize,
    //          second_y_distance: isize| {
    //             let first_x = x as isize + first_x_distance;
    //             let first_y = y as isize + first_y_distance;

    //             let second_x = x as isize + second_x_distance;
    //             let second_y = y as isize + second_y_distance;

    //             let x_or_z = self.height_difference(
    //                 first_x as usize,
    //                 first_y as usize,
    //                 second_x as usize,
    //                 second_y as usize,
    //             );

    //             let v = if calculate_x {
    //                 vec3(x_or_z, 1.0, 0.0)
    //             } else {
    //                 vec3(0.0, 1.0, x_or_z)
    //             };

    //             v.normalize() * adjacent_scale
    //         };

    //     // East
    //     let mut normal = get_adjacent_contribution(true, 0, 0, 1, 0);

    //     // West
    //     normal += get_adjacent_contribution(true, -1, 0, 0, 0);

    //     // North
    //     normal += get_adjacent_contribution(false, 0, 0, 0, 1);

    //     // South
    //     normal += get_adjacent_contribution(false, 0, -1, 0, 1);

    //     // For northeast, southeast, southwest, and northwest
    //     let diagonal_scale = vec3(0.1, 0.1, 0.1);
    //     let sqrt_two = 2.0.sqrt();

    //     let get_diagonal_contribution =
    //         |first_x_distance: isize,
    //          first_y_distance: isize,
    //          second_x_distance: isize,
    //          second_y_distance: isize| {
    //             let first_x = x as isize + first_x_distance;
    //             let first_y = y as isize + first_y_distance;

    //             let second_x = x as isize + second_x_distance;
    //             let second_y = y as isize + second_y_distance;

    //             let x_and_z = self.height_difference(
    //                 first_x as usize,
    //                 first_y as usize,
    //                 second_x as usize,
    //                 second_y as usize,
    //             ) / sqrt_two;

    //             vec3(x_and_z, sqrt_two, x_and_z).normalize() * diagonal_scale
    //         };

    //     // Northeast
    //     normal += get_diagonal_contribution(0, 0, 1, 1);

    //     // Southeast
    //     normal += get_diagonal_contribution(0, 0, 1, -1);

    //     // Southwest
    //     normal += get_diagonal_contribution(0, 0, -1, -1);

    //     // Northwest
    //     normal += get_diagonal_contribution(0, 0, -1, 1);

    //     normal
    // }

    pub fn surface_normal_at_xyz(&self, point: &Point3) -> Vec3 {
        self.surface_normal_at(&point.xz())
    }

    pub fn surface_normal_at(&self, point: &Point2) -> Vec3 {
        let denormalized_point = self.denormalize(point);

        let height = self
            .height_at(point)
            .expect("Tried to check surface normal at a point outside the grid.");

        // Use denormalized points here for extra precision.
        let east = usize_pt2(denormalized_point.x + 1, denormalized_point.y);
        let west = usize_pt2(denormalized_point.x - 1, denormalized_point.y);
        let north = usize_pt2(denormalized_point.x, denormalized_point.y + 1);
        let south = usize_pt2(denormalized_point.x, denormalized_point.y - 1);

        let neighbors = [east, west, north, south];

        let [height_west, height_east, height_north, height_south] =
            neighbors.map(|neighbor| self.height_at_denormalized(&neighbor).unwrap_or(height));

        let x_step_size = 1.0 / self.x_resolution() as f32;
        let z_step_size = 1.0 / self.z_resolution() as f32;

        let x_difference = (height_east - height_west) / (2.0 * x_step_size);
        let z_difference = (height_south - height_north) / (2.0 * z_step_size);

        vec3(x_difference, 1.0, z_difference).normalize()
    }

    pub fn height_at(&self, point: &Point2) -> Option<Height> {
        let usize_point2 = self.denormalize(point);
        self.height_at_denormalized(&usize_point2)
    }

    fn height_at_denormalized(&self, point: &UsizePoint2) -> Option<Height> {
        self.points.get(point).copied()
    }

    fn denormalize(&self, point: &Point2) -> UsizePoint2 {
        let denormalized = point.denormalize(&self.rect());
        denormalized.floor_to_usize_point2()
    }

    fn normalize(&self, domain_point: &UsizePoint2) -> Point3 {
        let height = *self.points.get(domain_point).unwrap();
        let normalized_domain_point = self.domain.normalize(domain_point);

        let x = normalized_domain_point.x;
        let y = height;
        let z = normalized_domain_point.y; // Notice we're using normalized_domain_point.y here. That's intentional. The y coordinate of the domain becomes the z coordinate when we switch to 3d.

        pt3(x, y, z)
    }

    fn height_difference(
        &self,
        first_x: usize,
        first_y: usize,
        second_x: usize,
        second_y: usize,
    ) -> f32 {
        let first = usize_pt2(first_x, first_y);
        let second = usize_pt2(second_x, second_y);

        let first_height = self.height_at_denormalized(&first);
        let second_height = self.height_at_denormalized(&second);

        // If first or second are outside the heightmap, return a height difference of 0.
        match (first_height, second_height) {
            (Some(first_height), Some(second_height)) => first_height - second_height,
            _ => 0.0,
        }
    }

    pub fn in_bounds(&self, point: &Point2) -> bool {
        point.within(0.0..=1.0)
    }

    pub fn update_height<GetNewHeight>(&mut self, point: &Point2, get_new_height: GetNewHeight)
    where
        GetNewHeight: Fn(Height) -> Height,
    {
        let height = self
            .height_at(point)
            .expect("Tried to update the height for a point outside the heightmap.");
        let new_height = get_new_height(height);
        let denormalized_point = self.denormalize(point);
        self.points.insert(denormalized_point, new_height);
    }

    fn rect(&self) -> Rect {
        let max_x = self.domain.max_x() as f32;
        let max_y = self.domain.max_y() as f32;

        Rect::from_corners(pt2(0.0, 0.0), pt2(max_x, max_y))
    }

    pub fn random_surface_point(&self, rand: &Rand) -> Point3 {
        let x = rand.zero_to_one();
        let z = rand.zero_to_one();
        let y = self
            .height_at(&pt2(x, z))
            .expect("Random particle position was outside the heightmap.");
        pt3(x, y, z)
    }
}

impl fmt::Debug for Heightmap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut rows = Vec::with_capacity(self.domain.num_rows());

        for y in 0..=self.domain.max_y() {
            let mut heights_in_row = Vec::with_capacity(self.domain.num_columns());
            for x in 0..=self.domain.max_x() {
                let point = usize_pt2(x, y);
                let height = self
                    .height_at_denormalized(&point)
                    .expect("Tried to get the height for a point outside the heightmap.");
                // Round to two decimal places.
                let formatted_height = format!("{:.2}", height);
                heights_in_row.push(formatted_height);
            }

            let row = "\t\t".to_string() + &heights_in_row.join(" ");
            rows.push(row);
        }

        let height_grid = rows.join("\n");

        write!(
            f,
            "Heightmap {{\n\twidth: {},\n\theight: {},\n\theights:\n{}\n}}",
            &self.domain.width, &self.domain.height, &height_grid
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn surface_normal_for_slope_rising_to_the_east() {
        let heightmap = Heightmap::new(3, |point| point.x);
        let _normal = heightmap.surface_normal_at(&pt2(0.5, 0.5));
        let _expected_normal = vec3(0.5, 1.0, 0.0).normalize();

        // assert_eq!(normal, expected_normal);
        // panic!();
    }

    #[test]
    fn surface_normal_for_slope_rising_to_the_south() {
        let heightmap = Heightmap::new(3, |point| point.y);
        let _normal = heightmap.surface_normal_at(&pt2(0.5, 0.5));
        let _expected_normal = vec3(0.0, 1.0, 0.5).normalize();

        // assert_eq!(normal, expected_normal);
        // panic!();
    }
}
