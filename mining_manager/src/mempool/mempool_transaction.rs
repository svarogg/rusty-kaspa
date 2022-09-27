use consensus_core::tx::{Transaction, TransactionId};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone)]
pub(crate) struct MempoolTransaction {
    pub(crate) transaction: Arc<Transaction>,
    pub(crate) parent_transactions_in_pool: Arc<HashMap<TransactionId, MempoolTransaction>>,
    pub(crate) is_high_priority: bool,
    pub(crate) added_at_daa_score: u64,
}

impl MempoolTransaction {
    pub fn new(
        transaction: Arc<Transaction>, parent_transactions_in_pool: Arc<HashMap<TransactionId, MempoolTransaction>>,
        is_high_priority: bool, added_at_daa_score: u64,
    ) -> Self {
        Self { transaction, parent_transactions_in_pool, is_high_priority, added_at_daa_score }
    }
}
