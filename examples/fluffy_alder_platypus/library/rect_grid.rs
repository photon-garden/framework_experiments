use crate::prelude::*;

#[derive(Debug)]
pub enum Side {
    Left,
    Right,
    Bottom,
    Top,
}

#[derive(Clone)]
pub struct Cell {
    pub rect: Rect,
    pub column_index: usize,
    pub row_index: usize,
}

impl Cell {
    pub fn contains(&self, point: &Point2) -> bool {
        self.rect.left() <= point.x
            && point.x <= self.rect.right()
            && self.rect.bottom() <= point.y
            && point.y <= self.rect.top()
    }
}

#[derive(Clone)]
pub struct RectGrid {
    pub cells: Vec<Cell>,
    pub num_columns: usize,
    pub num_rows: usize,
    pub container: Rect,
}

impl RectGrid {
    pub fn unit(num_columns: usize, num_rows: usize) -> RectGrid {
        RectGrid::new(Rect::unit(), num_columns, num_rows)
    }

    pub fn new(container: Rect, num_columns: usize, num_rows: usize) -> RectGrid {
        let num_cells_in_grid = num_columns * num_rows;
        let mut cells = Vec::with_capacity(num_cells_in_grid as usize);

        let column_width = container.w() as f32 / num_columns as f32;
        let row_height = container.h() as f32 / num_rows as f32;
        let cell_wh = vec2(column_width, row_height);

        for column_index in 0..num_columns {
            for row_index in 0..num_rows {
                let normalized_x = column_index as f32 / num_columns as f32;
                let normalized_y = row_index as f32 / num_rows as f32;

                let top_left = container.denormalize_x_y(normalized_x, normalized_y);
                let bottom_right = top_left + cell_wh;

                let rect = Rect::from_corners(top_left, bottom_right);
                let cell = Cell {
                    rect,
                    column_index,
                    row_index,
                };

                cells.push(cell);
            }
        }

        RectGrid {
            num_columns,
            num_rows,
            cells,
            container,
        }
    }

    pub fn index_for(&self, column: usize, row: usize) -> usize {
        let index = (self.num_rows * column) + row;
        let index_usize = index as usize;

        if index_usize > self.cells.len() {
            let message = format!(
                "Tried to get an index for coordinates ({}, {}), but the grid is only {}x{}.",
                column, row, self.num_columns, self.num_rows
            );
            panic!("{}", message);
        }

        index_usize
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Cell> {
        let index = self.index_for(x, y);
        self.cells.get(index)
    }

    pub fn random_edge_index(&self, sides: &[Side], rand: &Rand) -> usize {
        // let sides = vec![Side::Left, Side::Right, Side::Bottom, Side::Top];
        let random_side = rand.element(sides);

        let x;
        let y;

        match random_side {
            Side::Left => {
                x = 0;
                y = rand.range(0..=self.max_y());
            }
            Side::Right => {
                x = self.max_x();
                y = rand.range(0..=self.max_y());
            }
            Side::Bottom => {
                x = rand.range(0..=self.max_x());
                y = 0;
            }
            Side::Top => {
                x = rand.range(0..=self.max_x());
                y = self.max_y();
            }
        }

        self.index_for(x, y)
    }

    pub fn random_edge_index_from_any_side(&self, rand: &Rand) -> usize {
        let sides = vec![Side::Left, Side::Right, Side::Bottom, Side::Top];
        self.random_edge_index(&sides, rand)
    }

    pub fn max_x(&self) -> usize {
        self.num_columns - 1
    }

    pub fn max_y(&self) -> usize {
        self.num_rows - 1
    }

    pub fn find_xy(&self, point: &Point2) -> Option<&Cell> {
        self.cells.iter().find(|cell| cell.contains(point))
    }

    pub fn find_xy_cell_index(&self, point: &Point2) -> Option<usize> {
        self.cells.iter().position(|cell| cell.contains(point))
    }

    pub fn rows(&self) -> Vec<Vec<&Cell>> {
        let mut rows: Vec<Vec<&Cell>> = Vec::with_capacity(self.num_rows);

        // This implementation assumes that cells are sorted in order of
        // increasing row index and increasing column index.
        for cell in self.cells.iter() {
            let row_index = cell.row_index;

            if rows.get(row_index).is_none() {
                rows.push(Vec::with_capacity(self.num_columns));
            }

            let row = rows.get_mut(row_index).unwrap();
            row.push(cell);
        }

        rows
    }

    pub fn columns(&self) -> Vec<Vec<&Cell>> {
        let mut columns: Vec<Vec<&Cell>> = Vec::with_capacity(self.num_columns);

        // This implementation assumes that cells are sorted in order of
        // increasing row index and increasing column index.
        for cell in self.cells.iter() {
            let column_index = cell.column_index;

            if columns.get(column_index).is_none() {
                columns.push(Vec::with_capacity(self.num_rows));
            }

            let column = columns.get_mut(column_index).unwrap();
            column.push(cell);
        }

        columns
    }

    pub fn size(&self) -> usize {
        if self.num_rows != self.num_columns {
            panic!("Asked for size but this grid has different numbers of rows and columns");
        }

        self.num_columns
    }

    pub fn last_index(&self) -> usize {
        self.cells.last_index()
    }
}
