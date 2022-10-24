use crate::mempool::mempool_transaction::MempoolTransaction;
use consensus_core::tx::{TransactionOutpoint, UtxoEntry};
use std::collections::HashMap;

pub(crate) struct MempoolUTXOSet<'a> {
    pool_unspent_outputs: HashMap<TransactionOutpoint, UtxoEntry>,
    transactions_by_previous_outpoint: HashMap<TransactionOutpoint, MempoolTransaction>,
}

impl<'a> MempoolUTXOSet<'a> {
    pub fn new() -> Self {
        let pool_unspent_outputs = HashMap::new();
        let transactions_by_previous_outpoint = HashMap::new();

        Self { pool_unspent_outputs, transactions_by_previous_outpoint }
    }
}
