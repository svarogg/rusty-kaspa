use consensus_core::tx::{TransactionId, TransactionOutpoint};
use std::cell::RefCell;
use std::collections::HashMap;

pub struct OrphanPool {
    all_orphans: HashMap<TransactionId, RefCell<OrphanTransaction>>,
    orphans_by_previous_output: HashMap<TransactionOutpoint, RefCell<OrphanTransaction>>,
}

struct OrphanTransaction {}

impl OrphanPool {
    pub fn new() -> Self {
        let all_orphans = HashMap::new();
        let orphans_by_previous_output = HashMap::new();

        Self { all_orphans, orphans_by_previous_output }
    }
}
