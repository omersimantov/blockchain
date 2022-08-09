use super::{now, Transaction};
use secp256k1::hashes::{sha256, Hash};

#[derive(Debug)]
pub struct Block {
    pub version: u64,
    pub hash: String,
    pub prev_hash: String,
    pub merkleroot: String, // 256-bit unitive hash of all of the hashed nodes in the merkle tree
    pub timestamp: u64,
    pub nonce: u64, // Adjustable integer used to mine new blocks as part of the proof of work system
}

impl Block {
    pub fn new(prev_hash: String, transactions: Vec<Transaction>) -> Self {
        let timestamp = now();
        let merkleroot = sha256::Hash::hash(format!("{:?}", transactions).as_bytes()).to_string();
        let hash =
            sha256::Hash::hash(format!("{}{:?}{}", &prev_hash, &merkleroot, &timestamp).as_bytes())
                .to_string();

        Block {
            version: 1,
            hash,
            prev_hash,
            merkleroot,
            timestamp,
            nonce: 0,
        }
    }
}
