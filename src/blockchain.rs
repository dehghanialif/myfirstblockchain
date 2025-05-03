pub struct Transaction {
    sender: String,
    recipient: String,
    amount: u64,
}

pub struct Block {
    index: u64,
    timestamp: f64,
    transactions: Vec<Transaction>,
    proof: u64,
    previous_hash: String,
}

pub struct Blockchain {
    chain: Vec<Block>,
    current_transactions: Vec<Transaction>,
}

impl Blockchain {
    pub fn init() -> Self {
        Blockchain {
            chain: vec![],
            current_transactions: vec![],
        }
    }

    pub fn new_block(self: &Self) {}

    pub fn new_transaction(self: &Self) {}

    pub fn last_block(self: &Self) -> Option<&Block> {
        self.chain.last()
    }
}

pub fn hash() {}
