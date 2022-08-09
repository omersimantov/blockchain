use blockchainlib::{Blockchain, Wallet};

fn main() {
    // Create the blockchain
    let mut blockchain = Blockchain::new();

    // Create wallets for people
    let omer = Wallet::new();
    let leo = Wallet::new();
    let jenny = Wallet::new();
    let anne = Wallet::new();

    // Make Omer rich
    blockchain.add_transaction(leo.transact(172.0, omer.address.clone()));
    blockchain.add_transaction(jenny.transact(56.0, omer.address.clone()));
    blockchain.add_transaction(anne.transact(214.0, omer.address.clone()));

    // Mine a new block
    blockchain.mine_block();

    // Make Omer even richer
    blockchain.add_transaction(leo.transact(96.0, omer.address.clone()));
    blockchain.add_transaction(jenny.transact(44.0, omer.address.clone()));
    blockchain.add_transaction(anne.transact(27.0, omer.address.clone()));
    blockchain.add_transaction(anne.transact(20.0, omer.address.clone()));

    // Arbitrarily increase the mining difficulty to 6 leading zeros and mine another block
    blockchain.difficulty = "000000".to_owned();
    blockchain.mine_block();

    // Print the blockchain
    println!("\n{:?}", blockchain);
}
