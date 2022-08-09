use super::{now, Address, Transaction};
use secp256k1::hashes::{sha256, Hash};
use secp256k1::rand::rngs::OsRng;
use secp256k1::{Message, PublicKey, Secp256k1, SecretKey};

#[derive(Debug)]
pub struct Wallet {
    pub private_key: SecretKey,
    pub public_key: PublicKey,
    pub address: Address,
}

impl Wallet {
    pub fn new() -> Self {
        // Use the elliptic curve digital signature algorithm to generate a private and public key pair
        let secp = Secp256k1::new();
        let mut rng = OsRng::default();
        let private_key = SecretKey::new(&mut rng);
        let public_key = PublicKey::from_secret_key(&secp, &private_key);

        // Generate the wallet address by hashing the public key using the SHA-256 algorithm
        let address: Address = sha256::Hash::hash(public_key.to_string().as_bytes()).to_string();

        Wallet {
            private_key,
            public_key,
            address,
        }
    }

    pub fn transact(&self, amount: f64, address: Address) -> Transaction {
        // Create the digital signature of the transaction using the private key and the transaction data
        let timestamp = now();
        let secp = Secp256k1::new();
        let digest = sha256::Hash::hash(
            &format!("{}{}{}{}", amount, self.public_key, address, timestamp).as_bytes(),
        );
        let msg = Message::from_hashed_data::<sha256::Hash>(&digest);
        let signature = secp.sign_ecdsa(&msg, &self.private_key);

        Transaction::new(amount, self.public_key, address, timestamp, signature)
    }
}
