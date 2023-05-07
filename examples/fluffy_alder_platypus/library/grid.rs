use crate::prelude::*;

pub struct Grid {
    pub width: usize,
    pub height: usize,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Grid {
        Grid { width, height }
    }

    pub fn iterate_points(&self) -> impl Iterator<Item = UsizePoint2> {
        let x_range = 0..self.width;
        let height = self.height;

        x_range.into_iter().flat_map(move |x| {
            let y_range = 0..height;
            y_range.into_iter().map(move |y| usize_pt2(x, y))
        })
    }

    pub fn iterate_normalized_points(&self) -> impl Iterator<Item = Point2> + '_ {
        self.iterate_points()
            .map(|usize_point2| self.normalize(&usize_point2))
    }

    pub fn neighbors_around(&self, x: usize, y: usize) -> Vec<UsizePoint2> {
        let x = x as isize;
        let y = y as isize;
        let width = self.width as isize;
        let height = self.height as isize;

        let mut neighbors = vec![];

        let in_bounds = |x: isize, y: isize| (x >= 0 && x < width) && (y >= 0 && y < height);

        for x_offset in -1..=1 {
            for y_offset in -1..=1 {
                let neighbor_x = x - x_offset;
                let neighbor_y = y - y_offset;
                if in_bounds(neighbor_x, neighbor_y) {
                    let neighbor = usize_pt2(neighbor_x as usize, neighbor_y as usize);
                    neighbors.push(neighbor);
                }
            }
        }

        neighbors
    }

    pub fn normalize(&self, point: &UsizePoint2) -> Point2 {
        let x = point.x.normalize(0, self.max_x());
        let y = point.y.normalize(0, self.max_y());
        pt2(x, y)
    }

    pub fn in_bounds(&self, x: usize, y: usize) -> bool {
        // x and y are unsigned, so have to be 0 or greater.
        // No need to check that.
        let x_in_bounds = x < self.width;
        let y_in_bounds = y < self.height;

        x_in_bounds && y_in_bounds
    }

    pub fn max_x(&self) -> usize {
        self.width - 1
    }

    pub fn max_y(&self) -> usize {
        self.height - 1
    }

    pub fn num_rows(&self) -> usize {
        self.height
    }

    pub fn num_columns(&self) -> usize {
        self.width
    }
}
