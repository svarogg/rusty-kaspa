use consensus_core::tx::{TransactionId, ValidatedTransaction};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone)]
pub(crate) struct MempoolTransaction<'a> {
    pub(crate) transaction: Arc<ValidatedTransaction<'a>>,
    pub(crate) parent_transactions_in_pool: HashMap<TransactionId, MempoolTransaction<'a>>,
    pub(crate) is_high_priority: bool,
    pub(crate) added_at_daa_score: u64,
}

impl<'a> MempoolTransaction<'a> {
    pub fn new(
        transaction: Arc<ValidatedTransaction>,
        parent_transactions_in_pool: HashMap<TransactionId, MempoolTransaction>,
        is_high_priority: bool,
        added_at_daa_score: u64,
    ) -> Self {
        Self { transaction, parent_transactions_in_pool, is_high_priority, added_at_daa_score }
    }
}
