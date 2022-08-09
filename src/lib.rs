mod block;
mod blockchain;
mod transaction;
mod wallet;

use std::time::{SystemTime, UNIX_EPOCH};

pub type Address = String;

// A utility function to get the current number of seconds since epoch
pub fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub use block::Block;
pub use blockchain::Blockchain;
pub use transaction::Transaction;
pub use wallet::Wallet;
