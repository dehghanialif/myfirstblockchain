use hex::ToHex;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use std::time::{SystemTime, UNIX_EPOCH};
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub sender: String,
    pub recipient: String,
    pub amount: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct Block {
    pub index: usize,
    pub timestamp: f64,
    pub transactions: Vec<Transaction>,
    pub proof: u64,
    pub previous_hash: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Blockchain {
    chain: Vec<Block>,
    current_transactions: Vec<Transaction>,
    nodes: HashSet<String>,
}

impl Blockchain {
    pub fn init() -> Self {
        let mut blockchain = Blockchain {
            chain: vec![],
            current_transactions: vec![],
            nodes: HashSet::new(),
        };

        blockchain.new_block(100, Some(String::from("1")));

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

    pub fn new_transaction(
        self: &mut Self,
        sender: String,
        recipient: String,
        amount: u64,
    ) -> usize {
        let new_transaction = Transaction {
            sender,
            recipient,
            amount,
        };
        self.current_transactions.push(new_transaction);
        self.chain.len()
    }

    pub fn last_block(self: &Self) -> Block {
        self.chain.last().unwrap().clone()
    }

    pub fn proof_of_work(self: &Self, last_proof: u64) -> u64 {
        let mut proof: u64 = 0;
        while valid_proof(last_proof, proof) != true {
            proof += 1;
        }

        proof
    }

    pub fn register_node(self: &mut Self, address: String) {
        if let Ok(parsed_url) = Url::parse(&address) {
            if let Some(netloc) = parsed_url.host_str() {
                let port = parsed_url.port_or_known_default().unwrap_or(80);
                let full_address = format!("{}:{}", netloc, port);
                self.nodes.insert(full_address);
            }
        } else {
            println!("Invalid URL: {}", address);
        }
    }
}

pub fn valid_proof(last_proof: u64, proof: u64) -> bool {
    let guess = format!("{last_proof}{proof}").encode_hex();
    let guess_hash = hash(guess);
    guess_hash.ends_with("0000")
}

pub fn block_hash(block: &Block) -> String {
    let serialized = serde_json::to_string(block).unwrap();
    hash(serialized)
}

pub fn hash(content: String) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let blockchain = Blockchain::init();
        assert_eq!(blockchain.chain.len(), 1);
        assert_eq!(blockchain.last_block().proof, 100);
        assert_eq!(blockchain.last_block().previous_hash, String::from("1"));
        assert!(blockchain.current_transactions.is_empty());
    }

    #[test]
    fn test_new_transaction() {
        let mut blockchain = Blockchain::init();
        for i in 0..1000 {
            let sender = format!("Bob{i}");
            let recipient = format!("Alice{i}");
            let amount = i;
            blockchain.new_transaction(sender.clone(), recipient.clone(), amount);
            assert_eq!(
                blockchain
                    .current_transactions
                    .get(i as usize)
                    .unwrap()
                    .sender,
                sender
            );
            assert_eq!(
                blockchain
                    .current_transactions
                    .get(i as usize)
                    .unwrap()
                    .recipient,
                recipient
            );
            assert_eq!(
                blockchain
                    .current_transactions
                    .get(i as usize)
                    .unwrap()
                    .amount,
                amount
            );
        }
        assert_eq!(blockchain.current_transactions.len(), 1000);
    }

    #[test]
    fn test_new_block() {
        let mut blockchain = Blockchain::init();
        blockchain.new_transaction(String::from("Bob"), String::from("Alice"), 10);
        let block = blockchain.new_block(0, Some(String::from("1")));
        assert_eq!(blockchain.chain.len(), 2);
        assert_eq!(block.timestamp < get_unix_timestamp(), true);
        assert_eq!(block.transactions.len(), 1);
    }

    #[test]
    fn test_last_block() {
        let mut blockchain = Blockchain::init();
        assert_eq!(blockchain.last_block().proof, 100);
        assert_eq!(blockchain.last_block().previous_hash, String::from("1"));
    }
}
