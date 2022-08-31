use crate::mempool::{Mempool, MempoolTransaction};
use consensus_core::tx::{TransactionOutpoint, UtxoEntry};
use std::collections::HashMap;

pub struct MempoolUTXOSet<'a> {
    mempool: &'a Mempool,
    pool_unspent_outputs: HashMap<TransactionOutpoint, UtxoEntry>,
    transactions_by_previous_outpoint: HashMap<TransactionOutpoint, MempoolTransaction>,
}

impl<'a> MempoolUTXOSet<'a> {
    pub fn new(mempool: &'a Mempool) -> Self {
        let pool_unspent_outputs = HashMap::new();
        let transactions_by_previous_outpoint = HashMap::new();

        Self { mempool, pool_unspent_outputs, transactions_by_previous_outpoint }
    }
}
