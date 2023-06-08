use crate::prelude::*;

pub fn context_grid_xy() -> ContextGridPointGeneratorHeart {
    ContextGridPointGeneratorHeart {
        x_resolution: 4,
        y_resolution: 4,
        x_index: 0,
        y_index: 0,
        traverse: Traverse::RowByRow,
    }
}

pub struct ContextGridPointGeneratorHeart {
    x_resolution: usize,
    y_resolution: usize,
    x_index: usize,
    y_index: usize,
    traverse: Traverse,
}

#[derive(Clone, Debug)]
pub struct GridPoint2 {
    pub x_index: usize,
    pub y_index: usize,
    pub xy: Point2,
}

impl<Input, Context> GeneratorHeart<Input, GridPoint2, Context> for ContextGridPointGeneratorHeart
where
    Input: 'static,
    Context: Sized + 'static,
{
    fn generate_with_context(
        &mut self,
        _params: &GenerateWithContextParams<Input, Context>,
    ) -> GridPoint2 {
        let x_index = self.x_index;
        let y_index = self.y_index;

        let x = x_index as f32 / (self.x_resolution - 1) as f32;
        let y = y_index as f32 / (self.y_resolution - 1) as f32;

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

        GridPoint2 {
            x_index,
            y_index,
            xy: pt2(x, y),
        }
    }
}

impl ContextGridPointGeneratorHeart {
    pub fn x_resolution(mut self, x_resolution: usize) -> Self {
        self.x_resolution = x_resolution;
        self
    }

    pub fn y_resolution(mut self, y_resolution: usize) -> Self {
        self.y_resolution = y_resolution;
        self
    }

    pub fn row_by_row(mut self) -> Self {
        self.traverse = Traverse::RowByRow;
        self
    }

    pub fn column_by_column(mut self) -> Self {
        self.traverse = Traverse::ColumnByColumn;
        self
    }
}

enum Traverse {
    RowByRow,
    ColumnByColumn,
}

impl<Input> IntoContextGenerator<Input, GridPoint2> for ContextGridPointGeneratorHeart
where
    Input: 'static,
{
    fn into_context_generator(self) -> ContextGenerator<Input, GridPoint2> {
        self.without_context().into_context_generator()
    }
}
