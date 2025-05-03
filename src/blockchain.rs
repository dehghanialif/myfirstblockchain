use hex::ToHex;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Serialize)]
pub struct Transaction {
    sender: String,
    recipient: String,
    amount: u64,
}

#[derive(Clone, Serialize)]
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
            previous_hash: previous_hash.unwrap_or_else(|| {
                block_hash(self.chain.last().expect("Chain should not be empty"))
            }),
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

    pub fn proof_of_work(self: &Self, last_proof: u64) -> u64 {
        let mut proof: u64 = 0;
        while valid_proof(last_proof, proof) != true {
            proof += 1;
        }

        proof
    }
}

fn valid_proof(last_proof: u64, proof: u64) -> bool {
    let guess = format!("{last_proof}{proof}").encode_hex();
    let guess_hash = hash(guess);
    guess_hash.ends_with("0000")
}

pub fn block_hash(block: &Block) -> String {
    let serialized = serde_json::to_string(block).unwrap();
    hash(serialized)
}

fn hash(content: String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}

fn get_unix_timestamp() -> f64 {
    let now = SystemTime::now();
    let duration_since_epoch = now.duration_since(UNIX_EPOCH).unwrap();
    duration_since_epoch.as_secs() as f64
        + duration_since_epoch.subsec_micros() as f64 / 1_000_000.0
}
