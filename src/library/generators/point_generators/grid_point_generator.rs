use crate::prelude::*;

pub fn grid_xy() -> GridPointGenerator {
    GridPointGenerator {
        x_resolution: 4,
        y_resolution: 4,
        x_index: 0,
        y_index: 0,
        traverse: Traverse::RowByRow,
    }
}

pub struct GridPointGenerator {
    x_resolution: usize,
    y_resolution: usize,
    x_index: usize,
    y_index: usize,
    traverse: Traverse,
}

impl Generator for GridPointGenerator {
    type Output = Point2;

    fn generate(&mut self, _rand: &Rand) -> Point2 {
        let x = self.x_index as f32 / (self.x_resolution - 1) as f32;
        let y = self.y_index as f32 / (self.y_resolution - 1) as f32;

        match self.traverse {
            Traverse::RowByRow => {
                self.x_index += 1;
                if self.x_index >= self.x_resolution {
                    self.x_index = 0;
                    self.y_index += 1;
                    if self.y_index >= self.y_resolution {
                        self.y_index = 0;
                    }
                }
            }

            Traverse::ColumnByColumn => {
                self.y_index += 1;
                if self.y_index >= self.y_resolution {
                    self.y_index = 0;
                    self.x_index += 1;
                    if self.x_index >= self.x_resolution {
                        self.x_index = 0;
                    }
                }
            }
        }

        pt2(x, y)
    }
}

impl GridPointGenerator {
    pub fn x_resolution(mut self, x_resolution: usize) -> GridPointGenerator {
        self.x_resolution = x_resolution;
        self
    }

    pub fn y_resolution(mut self, y_resolution: usize) -> GridPointGenerator {
        self.y_resolution = y_resolution;
        self
    }

    pub fn row_by_row(mut self) -> GridPointGenerator {
        self.traverse = Traverse::RowByRow;
        self
    }

    pub fn column_by_column(mut self) -> GridPointGenerator {
        self.traverse = Traverse::ColumnByColumn;
        self
    }
}

enum Traverse {
    RowByRow,
    ColumnByColumn,
}
