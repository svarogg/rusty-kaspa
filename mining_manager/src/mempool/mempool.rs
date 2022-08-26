use crate::mempool::mempool_utxo_set::MempoolUTXOSet;
use crate::mempool::orphan_pool::OrphanPool;
use crate::mempool::transactions_pool::TransactionsPool;

pub struct Mempool {
    transactions_pool: TransactionsPool,
    orphan_pool: OrphanPool,
    mempool_utxo_set: MempoolUTXOSet,
}

impl Mempool {
    pub fn new() -> Self {
        let transactions_pool = TransactionsPool::new();
        let orphan_pool = OrphanPool::new();
        let mempool_utxo_set = MempoolUTXOSet::new();
        Self { transactions_pool, orphan_pool, mempool_utxo_set }
    }
}
