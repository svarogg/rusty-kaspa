use consensus_core::tx::{Transaction, TransactionOutpoint, UtxoEntry};
use std::collections::HashMap;

pub struct MempoolUTXOSet {
    pool_unspent_outputs: HashMap<TransactionOutpoint, UtxoEntry>,
    transactions_by_previous_outpoint: HashMap<TransactionOutpoint, Transaction>,
}

impl MempoolUTXOSet {
    pub fn new() -> Self {
        let pool_unspent_outputs = HashMap::new();
        let transactions_by_previous_outpoint = HashMap::new();

        Self { pool_unspent_outputs, transactions_by_previous_outpoint }
    }
}
