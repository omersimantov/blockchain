use super::Address;
use secp256k1::ecdsa::Signature;
use secp256k1::hashes::{sha256, Hash};
use secp256k1::{Message, PublicKey, Secp256k1};

#[derive(Debug, Clone)]
pub struct Transaction {
    pub amount: f64,
    pub sender: PublicKey,
    pub receiver: Address,
    pub timestamp: u64,
    pub signature: Signature,
}

impl Transaction {
    pub fn new(
        amount: f64,
        sender: PublicKey,
        receiver: Address,
        timestamp: u64,
        signature: Signature,
    ) -> Self {
        Transaction {
            amount,
            sender,
            receiver,
            timestamp,
            signature,
        }
    }

    pub fn is_valid(&self) -> bool {
        // Validate the transaction by matching the transaction data with the digital signature
        let secp = Secp256k1::new();
        let digest = sha256::Hash::hash(
            format!(
                "{}{}{}{}",
                self.amount, self.sender, self.receiver, self.timestamp
            )
            .to_string()
            .as_bytes(),
        );
        let msg = Message::from_hashed_data::<sha256::Hash>(&digest);
        secp.verify_ecdsa(&msg, &self.signature, &self.sender)
            .is_ok()
    }
}
