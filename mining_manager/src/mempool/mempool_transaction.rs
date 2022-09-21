use consensus_core::tx::{Transaction, TransactionId};
use std::collections::HashMap;

#[derive(Clone)]
pub(crate) struct MempoolTransaction<'a> {
    pub(crate) transaction: &'a Transaction,
    pub(crate) parent_transactions_in_pool: &'a HashMap<TransactionId, MempoolTransaction<'a>>,
    pub(crate) is_high_priority: bool,
    pub(crate) added_at_daa_score: u64,
}

impl<'a> MempoolTransaction<'a> {
    pub fn new(
        transaction: &'a Transaction, parent_transactions_in_pool: &'a HashMap<TransactionId, MempoolTransaction<'a>>,
        is_high_priority: bool, added_at_daa_score: u64,
    ) -> Self {
        Self { transaction, parent_transactions_in_pool, is_high_priority, added_at_daa_score }
    }
}
