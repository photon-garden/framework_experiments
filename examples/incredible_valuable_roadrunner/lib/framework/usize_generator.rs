use super::*;
use crate::prelude::*;
use std::ops::RangeInclusive;

pub enum UsizeGeneratorMode {
    SameNumberEveryTime(usize),
    UniformRandom { range: RangeInclusive<usize> },
}

impl Into<UsizeGenerator> for usize {
    fn into(self) -> UsizeGenerator {
        UsizeGenerator {
            mode: UsizeGeneratorMode::SameNumberEveryTime(self),
        }
    }
}

pub struct UsizeGenerator {
    mode: UsizeGeneratorMode,
}

impl UsizeGenerator {
    pub fn generate(&mut self, rand: &Rand) -> usize {
        match &self.mode {
            UsizeGeneratorMode::SameNumberEveryTime(number) => *number,
            UsizeGeneratorMode::UniformRandom { range } => rand.range(range.clone()),
        }
    }
}
