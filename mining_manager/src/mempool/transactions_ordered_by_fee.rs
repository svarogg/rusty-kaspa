use crate::mempool::mempool_transaction::MempoolTransaction;
use sorted_vec::SortedVec;
use std::cmp::Ordering;
use std::ops::Index;

pub(crate) struct TransactionsOrderedByFee {
    vec: SortedVec<TransactionOrderedByFee>,
}

impl Index<usize> for TransactionsOrderedByFee {
    type Output = MempoolTransaction;

    fn index(&self, index: usize) -> &Self::Output {
        &self.vec[index].transaction
    }
}

impl TransactionsOrderedByFee {
    pub fn new() -> Self {
        Self { vec: SortedVec::new() }
    }

    pub fn insert(&mut self, transaction: MempoolTransaction) -> usize {
        self.vec
            .insert(TransactionOrderedByFee::new(transaction))
    }

    pub fn len(&mut self) -> usize {
        self.vec.len()
    }
}

struct TransactionOrderedByFee {
    transaction: MempoolTransaction,
}

impl TransactionOrderedByFee {
    pub fn new(transaction: MempoolTransaction) -> Self {
        Self { transaction }
    }
}

impl Eq for TransactionOrderedByFee {}

impl PartialEq<Self> for TransactionOrderedByFee {
    fn eq(&self, other: &Self) -> bool {
        self.transaction
            .transaction
            .fee
            .eq(&other.transaction.transaction.fee)
    }
}

impl PartialOrd<Self> for TransactionOrderedByFee {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.transaction
            .transaction
            .fee
            .partial_cmp(&other.transaction.transaction.fee)
    }
}
impl Ord for TransactionOrderedByFee {
    fn cmp(&self, other: &Self) -> Ordering {
        self.transaction
            .transaction
            .fee
            .cmp(&other.transaction.transaction.fee)
    }
}

#[cfg(test)]
mod tests {
    use crate::mempool::transactions_ordered_by_fee::TransactionsOrderedByFee;
    use crate::mempool::MempoolTransaction;
    use consensus_core::subnets::SUBNETWORK_ID_NATIVE;
    use consensus_core::tx::Transaction;

    #[test]
    fn test_transactions_ordered_by_fee() {
        fn transaction_with_fee(fee: u64) -> MempoolTransaction {
            MempoolTransaction::new(Transaction::new(
                0,
                Vec::new(),
                Vec::new(),
                0,
                SUBNETWORK_ID_NATIVE,
                0,
                Vec::new(),
                fee,
                0,
            ))
        }

        let biggest = transaction_with_fee(100);
        let middle = transaction_with_fee(10);
        let smallest = transaction_with_fee(1);

        let mut transactions = TransactionsOrderedByFee::new();

        transactions.insert(middle.clone());
        transactions.insert(smallest.clone());
        transactions.insert(biggest.clone());

        assert_eq!(3, transactions.len());
        assert_eq!(smallest.transaction.fee, transactions[0].transaction.fee);
        assert_eq!(middle.transaction.fee, transactions[1].transaction.fee);
        assert_eq!(biggest.transaction.fee, transactions[2].transaction.fee);
    }
}
