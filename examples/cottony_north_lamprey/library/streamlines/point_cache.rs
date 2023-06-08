use crate::prelude::*;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;

type Key = (usize, usize);
type CacheEntry = (f32, f32, Point2);

pub struct PointCache {
    cache: HashMap<Key, Vec<CacheEntry>>,
}

impl PointCache {
    pub fn new() -> PointCache {
        PointCache {
            cache: HashMap::new(),
        }
    }

    pub fn add_point(
        &mut self,
        min_distance_for_seed_points: f32,
        min_distance_for_growth_points: f32,
        point: Point2,
    ) {
        let key = self.key_for(&point);
        let points_in_cell = match self.cache.entry(key) {
            Vacant(entry) => entry.insert(Vec::new()),
            Occupied(entry) => entry.into_mut(),
        };

        points_in_cell.push((
            min_distance_for_seed_points,
            min_distance_for_growth_points,
            point,
        ));
    }

    pub fn points_near(&self, point: &Point2) -> Option<&Vec<CacheEntry>> {
        let key = self.key_for(point);
        self.cache.get(&key)
    }

    pub fn key_for(&self, point: &Point2) -> (usize, usize) {
        let subdivision_factor = 100.0;
        let cell_x = point.x.times(subdivision_factor).round() as usize;
        let cell_y = point.y.times(subdivision_factor).round() as usize;

        (cell_x, cell_y)
    }
}
