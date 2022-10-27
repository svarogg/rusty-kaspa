use crate::mempool::mempool_transaction::MempoolTransaction;
use crate::mempool::transactions_ordered_by_fee::TransactionsOrderedByFee;
use crate::mempool::Mempool;
use consensus_core::tx::{Transaction, TransactionId, ValidatedTransaction};
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::sync::Arc;

pub(crate) struct TransactionsPool<'a> {
    all_transactions: HashMap<TransactionId, RefCell<MempoolTransaction<'a>>>,
    high_priority_transactions: HashMap<TransactionId, RefCell<MempoolTransaction<'a>>>,
    chained_transactions_by_parent_id: HashMap<TransactionId, Vec<RefCell<MempoolTransaction<'a>>>>,
    transactions_ordered_by_fee: TransactionsOrderedByFee<'a>,
}

impl<'a> TransactionsPool<'a> {
    pub fn new() -> Self {
        let all_transactions = HashMap::new();
        let high_priority_transactions = HashMap::new();
        let chained_transactions_by_parent_id = HashMap::new();
        let transactions_ordered_by_fee = TransactionsOrderedByFee::new();

        Self { all_transactions, high_priority_transactions, chained_transactions_by_parent_id, transactions_ordered_by_fee }
    }
}

impl<'a> TransactionsPool<'a> {
    pub(crate) fn add_transaction(
        &'a mut self,
        mempool: &Mempool,
        transaction: Arc<ValidatedTransaction>,
        parent_transactions_in_pool: HashMap<TransactionId, MempoolTransaction>,
        is_high_priority: bool,
    ) -> MempoolTransaction {
        let virtual_daa_score = mempool.consensus.virtual_daa_score();
        let transaction_id = transaction.id();

        let mempool_transaction: RefCell<MempoolTransaction> =
            RefCell::new(MempoolTransaction::new(transaction, parent_transactions_in_pool, is_high_priority, virtual_daa_score));

        self.all_transactions.insert(transaction_id, mempool_transaction);

        for (parent_transaction_id, parent_transaction) in parent_transactions_in_pool {
            let entry = self.chained_transactions_by_parent_id.entry(parent_transaction_id).or_insert(Vec::new());

            entry.push(mempool_transaction)
        }

        mempool_transaction
    }
}
