use consensus_core::tx::{Transaction, TransactionId};
use std::collections::HashMap;

#[derive(Clone)]
pub(crate) struct MempoolTransaction {
    pub(crate) transaction: Transaction,
    pub(crate) parent_transactions_in_pool: HashMap<TransactionId, MempoolTransaction>,
    pub(crate) is_high_prioriry: bool,
    pub(crate) added_at_daa_score: u64,
}

impl MempoolTransaction {
    pub fn new(
        transaction: Transaction, parent_transactions_in_pool: HashMap<TransactionId, MempoolTransaction>,
        is_high_prioriry: bool, added_at_daa_score: u64,
    ) -> Self {
        Self { transaction, parent_transactions_in_pool, is_high_prioriry, added_at_daa_score }
    }
}
