use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub struct Transaction {
    sender: String,
    recipient: String,
    amount: u64,
}

#[derive(Clone)]
pub struct Block {
    index: usize,
    timestamp: f64,
    transactions: Vec<Transaction>,
    proof: u64,
    previous_hash: String,
}

#[derive(Clone)]
pub struct Blockchain {
    chain: Vec<Block>,
    current_transactions: Vec<Transaction>,
}

impl Blockchain {
    pub fn init() -> Self {
        let mut blockchain = Blockchain {
            chain: vec![],
            current_transactions: vec![],
        };

        blockchain.new_block(100, Some(String::from("")));

        blockchain
    }

    pub fn new_block(self: &mut Self, proof: u64, previous_hash: Option<String>) -> Block {
        let block = Block {
            index: self.chain.len() + 1,
            timestamp: get_unix_timestamp(),
            transactions: self.current_transactions.clone(),
            proof: proof,
            previous_hash: previous_hash
                .unwrap_or_else(|| hash(self.chain.last().expect("Chain should not be empty"))),
        };

        self.current_transactions.clear();
        self.chain.push(block.clone());
        block
    }

    pub fn new_transaction(self: &mut Self, sender: String, recipient: String, amount: u64) {
        let new_transaction = Transaction {
            sender,
            recipient,
            amount,
        };
        self.current_transactions.push(new_transaction);
    }

    pub fn last_block(self: &Self) -> Option<&Block> {
        self.chain.last()
    }
}

pub fn hash(block: &Block) -> String {
    String::from("")
}

fn get_unix_timestamp() -> f64 {
    let now = SystemTime::now();
    let duration_since_epoch = now.duration_since(UNIX_EPOCH).unwrap();
    duration_since_epoch.as_secs() as f64
        + duration_since_epoch.subsec_micros() as f64 / 1_000_000.0
}
