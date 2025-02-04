use moka::sync::Cache;
use std::{cmp::Reverse, collections::BinaryHeap, sync::Arc};

use crate::processes::ghostdag::ordering::SortableBlock;

use hashes::Hash;

pub type BlockWindowHeap = BinaryHeap<Reverse<SortableBlock>>;

/// Reader API for `BlockWindowCacheStore`.
pub trait BlockWindowCacheReader {
    fn get(&self, hash: &Hash) -> Option<Arc<BlockWindowHeap>>;
}

pub type BlockWindowCacheStore = Cache<Hash, Arc<BlockWindowHeap>>;

impl BlockWindowCacheReader for BlockWindowCacheStore {
    #[inline(always)]
    fn get(&self, hash: &Hash) -> Option<Arc<BlockWindowHeap>> {
        self.get(hash)
    }
}
