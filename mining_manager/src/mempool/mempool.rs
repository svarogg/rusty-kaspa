use crate::mempool::config::MempoolConfig;
use crate::mempool::mempool_utxo_set::MempoolUTXOSet;
use crate::mempool::orphan_pool::OrphanPool;
use crate::mempool::transactions_pool::TransactionsPool;
use consensus::consensus::Consensus;
use consensus::params::Params;

use std::sync::Arc;

pub struct Mempool {
    config: MempoolConfig,
    transactions_pool: TransactionsPool,
    orphan_pool: OrphanPool,
    mempool_utxo_set: MempoolUTXOSet,
    consensus: Arc<Consensus>,
}

impl Mempool {
    pub fn new(consensus: Arc<Consensus>, params: &Params) -> Self {
        let config = MempoolConfig::default(params);

        let transactions_pool = TransactionsPool::new();
        let orphan_pool = OrphanPool::new();
        let mempool_utxo_set = MempoolUTXOSet::new();

        Self { config, consensus, transactions_pool, orphan_pool, mempool_utxo_set }
    }
}
