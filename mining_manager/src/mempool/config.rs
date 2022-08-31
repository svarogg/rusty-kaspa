use consensus::constants;
use consensus::params::Params;

const DEFAULT_MAXIMUM_TRANSACTION_COUNT: u64 = 1_000_000;

const DEFAULT_TRANSACTION_EXPIRE_INTERVAL_SECONDS: u64 = 60;
const DEFAULT_TRANSACTION_EXPIRE_SCAN_INTERVAL_SECONDS: u64 = 10;
const DEFAULT_ORPHAN_EXPIRE_INTERVAL_SECONDS: u64 = 60;
const DEFAULT_ORPHAN_EXPIRE_SCAN_INTERVAL_SECONDS: u64 = 10;

const DEFAULT_MAXIMUM_ORPHAN_TRANSACTION_MASS: u64 = 100000;
// DEFAULT_MAXIMUM_ORPHAN_TRANSACTION_COUNT should remain small as long as we have recursion in
// removeOrphans when removeRedeemers = true
const DEFAULT_MAXIMUM_ORPHAN_TRANSACTION_COUNT: u64 = 50;

// DEFAULT_MINIMUM_RELAY_TRANSACTION_FEE specifies the minimum transaction fee for a transaction to be accepted to
// the mempool and relayed. It is specified in sompi per 1kg (or 1000 grams) of transaction mass.
const DEFAULT_MINIMUM_RELAY_TRANSACTION_FEE: u64 = 1000;

// Standard transaction version range might be different from what consensus accepts, therefore
// we define separate values in mempool.
// However, currently there's exactly one transaction version, so mempool accepts the same version
// as consensus.
const DEFAULT_MINIMUM_STANDARD_TRANSACTION_VERSION: u16 = constants::MAXIMUM_TRANSACTION_VERSION;
const DEFAULT_MAXIMUM_STANDARD_TRANSACTION_VERSION: u16 = constants::MAXIMUM_TRANSACTION_VERSION;

pub struct MempoolConfig {
    maximum_transaction_count: u64,
    transaction_expire_interval_daascore: u64,
    transaction_expire_scan_interval_daascore: u64,
    transaction_expire_scan_interval_seconds: u64,
    orphan_expire_interval_daascore: u64,
    orphan_expire_scan_interval_daascore: u64,
    maximum_orphan_transaction_mass: u64,
    maximum_orphan_transaction_count: u64,
    accept_non_standard: bool,
    maximum_mass_per_block: u64,
    minimum_relay_transaction_fee: u64,
    minimum_standard_transaction_version: u16,
    maximum_standard_transaction_version: u16,
}

impl MempoolConfig {
    pub fn new(
        maximum_transaction_count: u64, transaction_expire_interval_daascore: u64,
        transaction_expire_scan_interval_daascore: u64, transaction_expire_scan_interval_seconds: u64,
        orphan_expire_interval_daascore: u64, orphan_expire_scan_interval_daascore: u64,
        maximum_orphan_transaction_mass: u64, maximum_orphan_transaction_count: u64, accept_non_standard: bool,
        maximum_mass_per_block: u64, minimum_relay_transaction_fee: u64, minimum_standard_transaction_version: u16,
        maximum_standard_transaction_version: u16,
    ) -> Self {
        Self {
            maximum_transaction_count,
            transaction_expire_interval_daascore,
            transaction_expire_scan_interval_daascore,
            transaction_expire_scan_interval_seconds,
            orphan_expire_interval_daascore,
            orphan_expire_scan_interval_daascore,
            maximum_orphan_transaction_mass,
            maximum_orphan_transaction_count,
            accept_non_standard,
            maximum_mass_per_block,
            minimum_relay_transaction_fee,
            minimum_standard_transaction_version,
            maximum_standard_transaction_version,
        }
    }

    pub fn default(params: &Params) -> Self {
        let target_blocks_per_second = 1000 / params.target_time_per_block; // 1000 for milliseconds. TODO: Use some wrapper class to avoid magic numbers

        Self {
            maximum_transaction_count: DEFAULT_MAXIMUM_TRANSACTION_COUNT,
            transaction_expire_interval_daascore: DEFAULT_TRANSACTION_EXPIRE_INTERVAL_SECONDS
                / target_blocks_per_second,
            transaction_expire_scan_interval_daascore: DEFAULT_TRANSACTION_EXPIRE_SCAN_INTERVAL_SECONDS
                / target_blocks_per_second,
            transaction_expire_scan_interval_seconds: DEFAULT_TRANSACTION_EXPIRE_SCAN_INTERVAL_SECONDS,
            orphan_expire_interval_daascore: DEFAULT_ORPHAN_EXPIRE_INTERVAL_SECONDS / target_blocks_per_second,
            orphan_expire_scan_interval_daascore: DEFAULT_ORPHAN_EXPIRE_SCAN_INTERVAL_SECONDS
                / target_blocks_per_second,
            maximum_orphan_transaction_mass: DEFAULT_MAXIMUM_ORPHAN_TRANSACTION_MASS,
            maximum_orphan_transaction_count: DEFAULT_MAXIMUM_ORPHAN_TRANSACTION_COUNT,
            accept_non_standard: params.relay_non_std_transactions,
            maximum_mass_per_block: params.max_block_mass,
            minimum_relay_transaction_fee: DEFAULT_MINIMUM_RELAY_TRANSACTION_FEE,
            minimum_standard_transaction_version: DEFAULT_MINIMUM_STANDARD_TRANSACTION_VERSION,
            maximum_standard_transaction_version: DEFAULT_MAXIMUM_STANDARD_TRANSACTION_VERSION,
        }
    }
}
