use crate::prelude::*;

pub struct StreamlineCollection {
    pub vector: Vec<Streamline>,
    pub point_cache: PointCache,
}

impl StreamlineCollection {
    pub fn new(vector: Vec<Streamline>) -> StreamlineCollection {
        let point_cache = PointCache::new();

        StreamlineCollection {
            vector,
            point_cache,
        }
    }
}
