// Define a structure for the Non-Fungible Token (NFT)
struct NFT {
    identifier: String,
    owner: String,
    market_value: f64,
}

// Define a structure for the Transaction which includes methods to process the transaction
struct Transaction<'a> {
    nft: &'a mut NFT,
    from_address: String,
    to_address: String,
    fee_discount_rate: f64,
    state: String,
    low_fee_eligible: bool,
}

impl<'a> Transaction<'a> {
    // Constructor to initialize a new Transaction with potential for a low fee flag
    fn new(nft: &'a mut NFT, from_address: String, to_address: String, fee_discount_rate: f64, low_fee_eligible: bool) -> Transaction<'a> {
        Transaction {
            nft,
            from_address,
            to_address,
            fee_discount_rate,
            state: "INITIATED".to_string(),
            low_fee_eligible,
        }
    }

    // Processes the transaction by applying a fee discount and transferring ownership
    fn process_transaction(&mut self) {
        println!("Processing transaction for NFT {} from {} to {}", self.nft.identifier, self.from_address, self.to_address);
        self.state = "PENDING".to_string();
        
        let base_fee_discount = self.nft.market_value * self.fee_discount_rate;
        let additional_discount = if self.low_fee_eligible {
            println!("Eligible for additional low fee discount.");
            self.nft.market_value * 0.02  // additional 2% discount
        } else {
            0.0
        };

        let total_discount = base_fee_discount + additional_discount;
        println!("Applying a total gas fee discount of ${:.2}", total_discount);

        if self.from_address == self.nft.owner {
            // Assuming transaction costs are covered and transaction is successful
            self.nft.owner = self.to_address.clone();
            self.state = "COMPLETED".to_string();
            println!("Transaction completed successfully. NFT {} is now owned by {}.", self.nft.identifier, self.to_address);
            println!("Remaining market value of NFT after fee subsidy: ${:.2}", self.nft.market_value - total_discount);
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
    let mut transaction = Transaction::new(&mut nft, from_address, to_address, 0.05, true);
    transaction.process_transaction();
}
