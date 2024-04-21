struct NFT {
    identifier: String,
    owner: String,
    market_value: f64,
}

struct Transaction<'a> {
    nft: &'a mut NFT,
    from_address: String,
    to_address: String,
    fee_discount_rate: f64,
    state: String,
}

impl<'a> Transaction<'a> {
    // Initializes a new Transaction
    fn new(nft: &'a mut NFT, from_address: String, to_address: String, fee_discount_rate: f64) -> Transaction<'a> {
        Transaction {
            nft,
            from_address,
            to_address,
            fee_discount_rate,
            state: "INITIATED".to_string(),
        }
    }

    // Processes the transaction
    fn process_transaction(&mut self) {
        println!("Processing transaction for NFT {} from {} to {}", self.nft.identifier, self.from_address, self.to_address);
        self.state = "PENDING".to_string();
        if self.from_address == self.nft.owner {
            let fee_discount = self.nft.market_value * self.fee_discount_rate;
            println!("Applying a gas fee discount of ${:.2}, reducing transaction costs.", fee_discount);

            // Assuming the transaction costs are covered and the transaction is successful
            self.nft.owner = self.to_address.clone();
            self.state = "COMPLETED".to_string();
            println!("Transaction completed successfully. NFT {} is now owned by {}.", self.nft.identifier, self.to_address);
            println!("Remaining market value of NFT after fee subsidy: ${:.2}", self.nft.market_value - fee_discount);
        } else {
            self.state = "FAILED".to_string();
            println!("Transaction failed with error: Invalid transaction: owner mismatch");
        }
    }
}

fn main() {
    let mut nft = NFT {
        identifier: "NFT#1".to_string(),
        owner: "address1".to_string(),
        market_value: 1000.0,
    };

    let from_address = "address1".to_string();
    let to_address = "address2".to_string();
    let mut transaction = Transaction::new(&mut nft, from_address, to_address, 0.05);
    transaction.process_transaction();
}
