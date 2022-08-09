use super::{Block, Transaction};
use secp256k1::hashes::{sha256, Hash};

#[derive(Debug)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub transaction_pool: Vec<Transaction>, // Transactions that have been validated but not processed yet
    pub difficulty: String, // Current difficulty of mining in hard-coded string format for this implementation
}

impl Blockchain {
    pub fn new() -> Self {
        // Create the genesis block
        let genesis_block = Block::new(String::new(), vec![]);

        Blockchain {
            blocks: vec![genesis_block], // Initialize the blockchain with the genesis block
            transaction_pool: vec![],    // No transactions to begin with
            difficulty: "0000".to_owned(), // Arbitrarily initialize the mining difficulty to 4 leading zeros
        }
    }

    // Proof of work system
    pub fn mine_block(&mut self) {
        if self.transaction_pool.is_empty() {
            return;
        }

        // Create a new block
        let mut block = Block::new(
            self.blocks.last().unwrap().hash.clone(),
            self.transaction_pool.clone(),
        );

        // Start mining
        println!(
            "\nCurrent difficulty: {}. \n\nMining block #{}... ",
            self.difficulty,
            self.blocks.len()
        );

        loop {
            // Calculate the block hash with the current nonce
            let digest = format!(
                "{}{:?}{}{}",
                &block.prev_hash, &block.merkleroot, &block.timestamp, &block.nonce
            );
            let hash = sha256::Hash::hash(digest.as_bytes()).to_string();

            // Check if the block hash starts with the required number of leading zeros
            if hash.starts_with(&self.difficulty) {
                // If so, add the block to the blockchain, empty the transaction pool, and stop the mining
                println!("Found nonce: {} -> hash: {}.", block.nonce, hash);
                self.blocks.push(block);
                self.transaction_pool.clear();
                println!("Added block to the blockchain.");
                break;
            } else {
                // Otherwise, increment the nonce by 1 and try again
                block.nonce += 1;
            }
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        // Validate the transaction before adding it to the transaction pool
        if !transaction.is_valid() {
            println!(
                "Transaction with signature {} is invalid.",
                transaction.signature
            );
            return;
        }

        self.transaction_pool.push(transaction);
    }
}
