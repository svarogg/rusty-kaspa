mod mempool_utxo_set;
mod orphan_pool;
mod transactions_ordered_by_fee;
mod transactions_pool;

use crate::mempool::mempool_utxo_set::MempoolUTXOSet;
use crate::mempool::orphan_pool::OrphanPool;
use crate::mempool::transactions_pool::TransactionsPool;
use consensus::consensus::Consensus;
use consensus_core::tx::{Transaction, TransactionId};
use std::collections::HashMap;
use std::sync::Arc;

pub struct Mempool {
    transactions_pool: TransactionsPool,
    orphan_pool: OrphanPool,
    mempool_utxo_set: MempoolUTXOSet,
    consensus: Arc<Consensus>,
}

impl Mempool {
    pub fn new(consensus: Arc<Consensus>) -> Self {
        let transactions_pool = TransactionsPool::new();
        let orphan_pool = OrphanPool::new();
        let mempool_utxo_set = MempoolUTXOSet::new();

        Self { transactions_pool, orphan_pool, mempool_utxo_set, consensus }
    }
}

#[derive(Clone)]
pub struct MempoolTransaction {
    transaction: Transaction,
    parent_transactions_in_pool: HashMap<TransactionId, MempoolTransaction>,
    is_high_prioriry: bool,
    added_at_daa_score: u64,
}

impl MempoolTransaction {
    pub(crate) fn new(
        transaction: Transaction, parent_transactions_in_pool: HashMap<TransactionId, MempoolTransaction>,
        is_high_prioriry: bool, added_at_daa_score: u64,
    ) -> Self {
        Self { transaction, parent_transactions_in_pool, is_high_prioriry, added_at_daa_score }
    }
}
