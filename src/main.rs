use chrono::Utc;
use crypto_hash::hex_digest;
use serde_derive::{Deserialize, Serialize};
use serde_json;

pub const PREFIX: &str = "00";

#[derive(Serialize, Deserialize)]
pub struct Transaction {
    pub transaction_id: String,
    pub transaction_timestamp: i64,
    pub transaction_details: String,
}

#[derive(Serialize, Deserialize)]
pub struct Block {
    pub block_number: u64,
    block_timestamp: i64,
    pub block_nonce: u64,
    transaction_list: Vec<Transaction>,
    pub previous_block_hash: String,
}


impl Block {
    pub fn genesis() -> Self {
        let transaction = Transaction {
            transaction_id: String::from("1"),
            transaction_details: String::from("Transaction from the genesis method"),
            transaction_timestamp: Utc::now().timestamp(),
        };
        Block {
            block_number: 1,
            block_timestamp: Utc::now().timestamp(),
            block_nonce: 0,
            transaction_list: vec![transaction],
            previous_block_hash: String::from("1"),
        }
    }

    pub fn serialize_block(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn generate_hash(block: &Block) -> String {
        hex_digest(crypto_hash::Algorithm::SHA256, block.serialize_block().as_bytes())
    }

    pub fn is_block_valid(hash: &str, prefix:&str) -> bool {
        hash.starts_with(prefix)
    }

    pub fn new(transactions: Vec<Transaction>, previous_block: &Block) -> Block {
        Block {
            block_number: previous_block.block_number + 1,
            block_timestamp: Utc::now().timestamp(),
            block_nonce: 0,
            transaction_list: transactions,
            previous_block_hash: Self::generate_hash(previous_block),
        }
    }

    pub fn mine_new_block(candidate_block: &mut Block, prefix: &str) {
        while !Self::is_block_valid(&Self::generate_hash(candidate_block), prefix) {
            println!("{}", candidate_block.block_nonce);
            candidate_block.block_nonce +=1;
        }
    }
}

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_genesis_block() {
        let p2p_bc: Vec<Block> = vec![Block::genesis()];
        assert_eq!(p2p_bc[0].block_number, 1);
        assert_eq!(p2p_bc[0].transaction_list[0].transaction_details, "Transaction from the genesis method");
    }

    #[test]
    fn  test_new_block() {
        let mut p2p_bc: Vec<Block> = vec![Block::genesis()];
        let new_transaction = Transaction {
            transaction_id: String::from("1"),
            transaction_timestamp: 0,
            transaction_details: String::from("new transaction block"),
        };

        let mut new_block = Block::new(vec![new_transaction], &p2p_bc[p2p_bc.len() - 1]);
        Block::mine_new_block(&mut new_block, &PREFIX);
        p2p_bc.push(new_block);
        assert_eq!(p2p_bc.len(), 2);
        assert_eq!(p2p_bc[p2p_bc.len() - 1]. transaction_list[0].transaction_details, "new transaction block");
    }    
}