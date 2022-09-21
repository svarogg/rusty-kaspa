use crate::mempool::mempool_transaction::MempoolTransaction;
use sorted_vec::SortedVec;
use std::cmp::Ordering;
use std::ops::Index;

pub(crate) struct TransactionsOrderedByFee<'a> {
    vec: SortedVec<TransactionOrderedByFee<'a>>,
}

impl<'a> Index<usize> for TransactionsOrderedByFee<'a> {
    type Output = &'a MempoolTransaction<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.vec[index].transaction
    }
}

impl<'a> TransactionsOrderedByFee<'a> {
    pub fn new() -> Self {
        Self { vec: SortedVec::new() }
    }

    pub fn insert(&mut self, transaction: &'a MempoolTransaction) -> usize {
        self.vec
            .insert(TransactionOrderedByFee::new(transaction))
    }

    pub fn len(&mut self) -> usize {
        self.vec.len()
    }
}

struct TransactionOrderedByFee<'a> {
    transaction: &'a MempoolTransaction<'a>,
}

impl<'a> TransactionOrderedByFee<'a> {
    pub fn new(transaction: &'a MempoolTransaction) -> Self {
        Self { transaction }
    }
}

impl<'a> Eq for TransactionOrderedByFee<'a> {}

impl<'a> PartialEq<Self> for TransactionOrderedByFee<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.transaction
            .transaction
            .fee
            .eq(&other.transaction.transaction.fee)
    }
}

impl<'a> PartialOrd<Self> for TransactionOrderedByFee<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.transaction
            .transaction
            .fee
            .partial_cmp(&other.transaction.transaction.fee)
    }
}
impl<'a> Ord for TransactionOrderedByFee<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.transaction
            .transaction
            .fee
            .cmp(&other.transaction.transaction.fee)
    }
}

#[cfg(test)]
mod tests {
    use crate::mempool::mempool_transaction::MempoolTransaction;
    use crate::mempool::transactions_ordered_by_fee::TransactionsOrderedByFee;
    use consensus_core::subnets::SUBNETWORK_ID_NATIVE;
    use consensus_core::tx::Transaction;
    use std::collections::HashMap;

    #[test]
    fn test_transactions_ordered_by_fee() {
        fn transaction_with_fee(fee: u64) -> MempoolTransaction<'static> {
            MempoolTransaction::new(
                Transaction::new(0, Vec::new(), Vec::new(), 0, SUBNETWORK_ID_NATIVE, 0, Vec::new(), fee, 0),
                &HashMap::new(),
                false,
                0,
            )
        }

        let biggest = transaction_with_fee(100);
        let middle = transaction_with_fee(10);
        let smallest = transaction_with_fee(1);

        let mut transactions = TransactionsOrderedByFee::new();

        transactions.insert(&middle);
        transactions.insert(&smallest);
        transactions.insert(&biggest);

        assert_eq!(3, transactions.len());
        assert_eq!(smallest, transactions[0]);
        assert_eq!(middle, transactions[1]);
        assert_eq!(biggest, transactions[2]);
    }
}
