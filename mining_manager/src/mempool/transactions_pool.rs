use crate::mempool::transactions_ordered_by_fee::TransactionsOrderedByFee;
use crate::mempool::MempoolTransaction;
use consensus_core::tx::TransactionId;
use std::collections::HashMap;

pub struct TransactionsPool {
    all_transactions: HashMap<TransactionId, MempoolTransaction>,
    high_priority_transactions: HashMap<TransactionId, MempoolTransaction>,
    chained_transactions_by_parent_id: HashMap<TransactionId, Vec<MempoolTransaction>>,
    transactions_ordered_by_fee: TransactionsOrderedByFee,
}

impl TransactionsPool {
    pub fn new() -> Self {
        let all_transactions = HashMap::new();
        let high_priority_transactions = HashMap::new();
        let chained_transactions_by_parent_id = HashMap::new();
        let transactions_ordered_by_fee = TransactionsOrderedByFee::new();

        Self {
            all_transactions,
            high_priority_transactions,
            chained_transactions_by_parent_id,
            transactions_ordered_by_fee,
        }
    }
}
