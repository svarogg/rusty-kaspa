mod mempool_utxo_set;
mod orphan_pool;
mod transactions_ordered_by_fee;
mod transactions_pool;

use crate::mempool::mempool_utxo_set::MempoolUTXOSet;
use crate::mempool::orphan_pool::OrphanPool;
use crate::mempool::transactions_pool::TransactionsPool;
use consensus_core::tx::Transaction;

pub struct Mempool {
    transactions_pool: TransactionsPool,
    orphan_pool: OrphanPool,
    mempool_utxo_set: MempoolUTXOSet,
}

#[derive(Clone)]
pub struct MempoolTransaction {
    transaction: Transaction,
}

impl MempoolTransaction {
    pub(crate) fn new(transaction: Transaction) -> Self {
        Self { transaction }
    }
}

impl Mempool {
    pub fn new() -> Self {
        let transactions_pool = TransactionsPool::new();
        let orphan_pool = OrphanPool::new();
        let mempool_utxo_set = MempoolUTXOSet::new();

        Self { transactions_pool, orphan_pool, mempool_utxo_set }
    }
}
