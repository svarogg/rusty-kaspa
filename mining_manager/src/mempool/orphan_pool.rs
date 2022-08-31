use crate::mempool::Mempool;
use consensus_core::tx::{TransactionId, TransactionOutpoint};
use std::collections::HashMap;

pub struct OrphanPool<'a> {
    mempool: &'a Mempool,
    all_orphans: HashMap<TransactionId, OrphanTransaction>,
    orphans_by_previous_output: HashMap<TransactionOutpoint, OrphanTransaction>,
}

struct OrphanTransaction {}

impl<'a> OrphanPool<'a> {
    pub fn new(mempool: &'a Mempool) -> Self {
        let all_orphans = HashMap::new();
        let orphans_by_previous_output = HashMap::new();

        Self { mempool, all_orphans, orphans_by_previous_output }
    }
}
