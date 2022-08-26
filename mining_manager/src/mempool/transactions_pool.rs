use consensus_core::tx::{Transaction, TransactionId};
use std::collections::HashMap;

pub struct TransactionsPool {
    all_transactions: HashMap<TransactionId, Transaction>,
    high_priority_transactions: HashMap<TransactionId, Transaction>,
    chained_transactions_by_parent_id: HashMap<TransactionId, Vec<Transaction>>,
    // TODO: transactions_ordered_by_fee: TransactionsOrderedByFee,
}

impl TransactionsPool {
    pub fn new() -> Self {
        let all_transactions = HashMap::new();
        let high_priority_transactions = HashMap::new();
        let chained_transactions_by_parent_id = HashMap::new();

        Self { all_transactions, high_priority_transactions, chained_transactions_by_parent_id }
    }
}
