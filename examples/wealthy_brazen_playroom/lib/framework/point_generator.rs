use crate::prelude::*;
use std::ops::RangeInclusive;

use super::f32_generator::F32Generator;

pub fn uniform_random_xy() -> PointGenerator {
    PointGenerator {
        mode: PointGeneratorMode::UniformRandom {
            x_range: 0.0..=1.0,
            y_range: 0.0..=1.0,
        },
    }
}

pub fn grid_xy() -> PointGenerator {
    PointGenerator {
        mode: PointGeneratorMode::Grid {
            x_resolution: 4,
            y_resolution: 4,
            x_index: 0,
            y_index: 0,
            traverse: GridTraverse::RowByRow,
        },
    }
}

pub struct PointGenerator {
    mode: PointGeneratorMode,
}

enum PointGeneratorMode {
    SamePointEveryTime(Point2),
    UniformRandom {
        x_range: RangeInclusive<f32>,
        y_range: RangeInclusive<f32>,
    },
    Grid {
        x_resolution: usize,
        y_resolution: usize,
        x_index: usize,
        y_index: usize,
        traverse: GridTraverse,
    },
    Jitter {
        point_generator: Box<PointGenerator>,
        x_amount: F32Generator,
        y_amount: F32Generator,
    },
}

impl PointGenerator {
    pub fn generate(&mut self, rand: &Rand) -> Point2 {
        match &mut self.mode {
            PointGeneratorMode::SamePointEveryTime(point) => *point,

            PointGeneratorMode::UniformRandom { x_range, y_range } => {
                let x = rand.zero_to_one().denormalize_to_range(x_range);
                let y = rand.zero_to_one().denormalize_to_range(y_range);
                pt2(x, y)
            }

            PointGeneratorMode::Grid {
                x_resolution,
                y_resolution,
                x_index,
                y_index,
                traverse,
            } => {
                let x = *x_index as f32 / (*x_resolution - 1) as f32;
                let y = *y_index as f32 / (*y_resolution - 1) as f32;

                match traverse {
                    GridTraverse::RowByRow => {
                        *x_index += 1;
                        if *x_index >= *x_resolution {
                            *x_index = 0;
                            *y_index += 1;
                            if *y_index >= *y_resolution {
                                *y_index = 0;
                            }
                        }
                    }

                    GridTraverse::ColumnByColumn => {
                        *y_index += 1;
                        if *y_index >= *y_resolution {
                            *y_index = 0;
                            *x_index += 1;
                            if *x_index >= *x_resolution {
                                *x_index = 0;
                            }
                        }
                    }
                }

                pt2(x, y)
            }

            PointGeneratorMode::Jitter {
                point_generator,
                x_amount,
                y_amount,
            } => {
                let point = point_generator.generate(rand);

                let x_jitter = rand.zero_to_one() * x_amount.generate(rand);
                let y_jitter = rand.zero_to_one() * y_amount.generate(rand);

                let x = point.x + x_jitter;
                let y = point.y + y_jitter;

                pt2(x, y)
            }
        }
    }

    pub fn x_resolution(mut self, x_resolution: usize) -> PointGenerator {
        match self.mode {
            PointGeneratorMode::Grid {
                y_resolution,
                x_index,
                y_index,
                traverse,
                ..
            } => {
                self.mode = PointGeneratorMode::Grid {
                    x_resolution,
                    y_resolution,
                    x_index,
                    y_index,
                    traverse,
                };
                self
            }

            _ => self,
        }
    }

    pub fn y_resolution(mut self, y_resolution: usize) -> PointGenerator {
        match self.mode {
            PointGeneratorMode::Grid {
                x_resolution,
                x_index,
                y_index,
                traverse,
                ..
            } => {
                self.mode = PointGeneratorMode::Grid {
                    x_resolution,
                    y_resolution,
                    x_index,
                    y_index,
                    traverse,
                };
                self
            }
            _ => self,
        }
    }

    pub fn row_by_row(mut self) -> PointGenerator {
        match self.mode {
            PointGeneratorMode::Grid {
                x_resolution,
                y_resolution,
                x_index,
                y_index,
                ..
            } => {
                self.mode = PointGeneratorMode::Grid {
                    x_resolution,
                    y_resolution,
                    x_index,
                    y_index,
                    traverse: GridTraverse::RowByRow,
                };
                self
            }

            _ => self,
        }
    }

    pub fn column_by_column(mut self) -> PointGenerator {
        match self.mode {
            PointGeneratorMode::Grid {
                x_resolution,
                y_resolution,
                x_index,
                y_index,
                ..
            } => {
                self.mode = PointGeneratorMode::Grid {
                    x_resolution,
                    y_resolution,
                    x_index,
                    y_index,
                    traverse: GridTraverse::ColumnByColumn,
                };
                self
            }

            _ => self,
        }
    }

    pub fn jitter(self) -> PointGenerator {
        let mode = PointGeneratorMode::Jitter {
            point_generator: Box::new(self),
            x_amount: 0.01.into(),
            y_amount: 0.01.into(),
        };

        PointGenerator { mode }
    }

    pub fn x_jitter(mut self, x_amount: impl Into<F32Generator>) -> PointGenerator {
        match self.mode {
            PointGeneratorMode::Jitter {
                point_generator,
                y_amount,
                ..
            } => {
                self.mode = PointGeneratorMode::Jitter {
                    point_generator,
                    x_amount: x_amount.into(),
                    y_amount,
                };
                self
            }

            _ => self,
        }
    }

    pub fn y_jitter(mut self, y_amount: impl Into<F32Generator>) -> PointGenerator {
        match self.mode {
            PointGeneratorMode::Jitter {
                point_generator,
                x_amount,
                ..
            } => {
                self.mode = PointGeneratorMode::Jitter {
                    point_generator,
                    x_amount,
                    y_amount: y_amount.into(),
                };
                self
            }

            _ => self,
        }
    }
}

impl Default for PointGenerator {
    fn default() -> Self {
        PointGenerator {
            mode: PointGeneratorMode::SamePointEveryTime(pt2(0.0, 0.0)),
        }
    }
}

impl Into<PointGenerator> for Point2 {
    fn into(self) -> PointGenerator {
        PointGenerator {
            mode: PointGeneratorMode::SamePointEveryTime(self),
        }
    }
}

enum GridTraverse {
    RowByRow,
    ColumnByColumn,
}
