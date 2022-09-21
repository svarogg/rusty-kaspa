use crate::mempool::config::MempoolConfig;
use crate::mempool::mempool_utxo_set::MempoolUTXOSet;
use crate::mempool::orphan_pool::OrphanPool;
use crate::mempool::transactions_pool::TransactionsPool;
use consensus::consensus::Consensus;
use consensus::params::Params;
use std::sync::Arc;

mod config;
mod mempool_transaction;
mod mempool_utxo_set;
mod orphan_pool;
mod transactions_ordered_by_fee;
mod transactions_pool;

pub struct Mempool<'a> {
    config: MempoolConfig,
    transactions_pool: TransactionsPool<'a>,
    orphan_pool: OrphanPool,
    mempool_utxo_set: MempoolUTXOSet<'a>,
    pub(crate) consensus: Arc<Consensus>,
}

impl<'a> Mempool<'a> {
    pub fn new(consensus: Arc<Consensus>, params: &Params) -> Self {
        let config = MempoolConfig::default(params);

        let transactions_pool = TransactionsPool::new();
        let orphan_pool = OrphanPool::new();
        let mempool_utxo_set = MempoolUTXOSet::new();

        Self { config, consensus, transactions_pool, orphan_pool, mempool_utxo_set }
    }
}
