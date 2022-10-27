use crate::mempool::mempool_transaction::MempoolTransaction;
use sorted_vec::SortedVec;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::ops::Index;

pub(crate) struct TransactionsOrderedByFee<'a> {
    vec: SortedVec<TransactionOrderedByFee<'a>>,
}

impl<'a> Index<usize> for TransactionsOrderedByFee<'a> {
    type Output = RefCell<MempoolTransaction<'a>>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.vec[index].transaction
    }
}

impl<'a> TransactionsOrderedByFee<'a> {
    pub fn new() -> Self {
        Self { vec: SortedVec::new() }
    }

    pub fn insert(&mut self, transaction: RefCell<MempoolTransaction>) -> usize {
        self.vec.insert(TransactionOrderedByFee::new(transaction))
    }

    pub fn len(&mut self) -> usize {
        self.vec.len()
    }
}

struct TransactionOrderedByFee<'a> {
    transaction: RefCell<MempoolTransaction<'a>>,
}

impl<'a> TransactionOrderedByFee<'a> {
    pub fn new(transaction: RefCell<MempoolTransaction>) -> Self {
        Self { transaction }
    }
}

impl<'a> Eq for TransactionOrderedByFee<'a> {}

impl<'a> PartialEq<Self> for TransactionOrderedByFee<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.transaction.borrow().transaction.calculated_fee.eq(&other.transaction.borrow().transaction.calculated_fee)
    }
}

impl<'a> PartialOrd<Self> for TransactionOrderedByFee<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.transaction.borrow().transaction.calculated_fee.partial_cmp(&other.transaction.borrow().transaction.calculated_fee)
    }
}
impl<'a> Ord for TransactionOrderedByFee<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.transaction.borrow().transaction.calculated_fee.cmp(&other.transaction.borrow().transaction.calculated_fee)
    }
}

#[cfg(test)]
mod tests {
    use crate::mempool::mempool_transaction::MempoolTransaction;
    use crate::mempool::transactions_ordered_by_fee::TransactionsOrderedByFee;
    use consensus_core::subnets::SUBNETWORK_ID_NATIVE;
    use consensus_core::tx::{PopulatedTransaction, Transaction, ValidatedTransaction};
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::sync::Arc;

    #[test]
    fn test_transactions_ordered_by_fee() {
        fn transaction_with_fee<'a>(fee: u64) -> RefCell<MempoolTransaction<'a>> {
            RefCell::new(MempoolTransaction::new(
                Arc::new(ValidatedTransaction::new(
                    PopulatedTransaction::new(
                        &Transaction::new(0, Vec::new(), Vec::new(), 0, SUBNETWORK_ID_NATIVE, 0, Vec::new()),
                        Vec::new(),
                    ),
                    fee,
                )),
                HashMap::new(),
                false,
                0,
            ))
        }

        let biggest = transaction_with_fee(100);
        let middle = transaction_with_fee(10);
        let smallest = transaction_with_fee(1);

        let mut transactions = TransactionsOrderedByFee::new();

        transactions.insert(middle);
        transactions.insert(smallest);
        transactions.insert(biggest);

        assert_eq!(3, transactions.len());
        assert_eq!(smallest.borrow().transaction.calculated_fee, transactions[0].borrow().transaction.calculated_fee);
        assert_eq!(middle.borrow().transaction.calculated_fee, transactions[1].borrow().transaction.calculated_fee);
        assert_eq!(biggest.borrow().transaction.calculated_fee, transactions[2].borrow().transaction.calculated_fee);
    }
}
