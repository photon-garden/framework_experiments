use crate::prelude::*;
use std::collections::HashMap;

type Key = [usize; 2];

pub struct SpatialHash<'item, Item> {
    cell_size: f32,
    buckets: HashMap<Key, Vec<&'item Item>>,
}

impl<'item, Item> SpatialHash<'item, Item> {
    pub fn new(cell_size: f32) -> SpatialHash<'item, Item> {
        SpatialHash {
            cell_size,
            buckets: HashMap::new(),
        }
    }

    pub fn neighbors_with_bounding_box(
        &mut self,
        bounding_box: Rect,
    ) -> impl Iterator<Item = &&'item Item> {
        self.iterate_keys_in_bounding_box(bounding_box)
            .filter_map(|key| self.buckets.get(&key))
            .flatten()
    }

    pub fn insert_with_bounding_box(&mut self, bounding_box: Rect, item: &'item Item) {
        for key in self.iterate_keys_in_bounding_box(bounding_box) {
            self.insert_item(key, item);
        }
    }

    pub fn insert_xy(&mut self, xy: Point2, item: &'item Item) {
        let key = get_key(self.cell_size, xy);
        self.insert_item(key, item);
    }

    fn insert_item(&mut self, key: Key, item: &'item Item) {
        self.buckets.entry(key).or_default().push(item);
    }

    fn iterate_keys_in_bounding_box(&self, bounding_box: Rect) -> impl Iterator<Item = Key> {
        let bottom_left = bounding_box.bottom_left();
        let top_right = bounding_box.top_right();

        // x and y here refer to bucket indexes. Not the x and y space
        // of the actual artwork.
        let [min_x, min_y] = get_key(self.cell_size, bottom_left);
        let [max_x, max_y] = get_key(self.cell_size, top_right);

        (min_x..=max_x)
            .into_iter()
            .flat_map(move |x| (min_y..=max_y).into_iter().map(move |y| [x, y]))
    }
}

impl<'item, Item> SpatialHash<'item, Item>
where
    Item: GetBoundingBox,
{
    pub fn from_items(cell_size: f32, items: Vec<&'item Item>) -> SpatialHash<'item, Item> {
        let buckets = HashMap::new();
        let mut spatial_hash = SpatialHash { cell_size, buckets };
        for item in items {
            spatial_hash.insert(item);
        }
        spatial_hash
    }

    pub fn insert(&mut self, item: &'item Item) {
        let bounding_box = item.bounding_box();
        self.insert_with_bounding_box(bounding_box, item);
    }

    pub fn neighbors(&mut self, item: &'item Item) -> impl Iterator<Item = &&'item Item> {
        let bounding_box = item.bounding_box();
        self.iterate_keys_in_bounding_box(bounding_box)
            .filter_map(|key| self.buckets.get(&key))
            .flatten()
    }
}

pub trait GetBoundingBox {
    fn bounding_box(&self) -> Rect;
}

fn get_key(cell_size: f32, xy: Point2) -> Key {
    [
        hash_coordinate(cell_size, xy.x),
        hash_coordinate(cell_size, xy.y),
    ]
}

fn hash_coordinate(cell_size: f32, coordinate: f32) -> usize {
    coordinate.divided_by(cell_size).floor() as usize
}
