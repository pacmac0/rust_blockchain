use std::fmt::{ self, Debug, Formatter };
use super::*;

pub struct Block{
    pub index: u32,
    pub timestamp: u128,
    pub block_hash: Hash,
    pub prev_block_hash: Hash,
    pub nonce: u64,
    pub transactions: String, //Vec<Transaction>,
    pub difficulty: u128,
}

impl Block {
    pub fn new (index: u32, timestamp: u128, prev_block_hash: Hash, transactions: String, nonce: u64, difficulty: u128) -> Self {
        Block {
            index,
            timestamp,
            prev_block_hash,
            block_hash: vec![0; 32],
            nonce,
            difficulty,
            transactions,
        }
    }

    pub fn mine (&mut self) {
        for nonce_candidate in 0..(u64::max_value()) {
            self.nonce = nonce_candidate;
            let block_hash = self.hash();
            if check_difficulty(&block_hash, self.difficulty) {
                self.block_hash = block_hash;
                return;
            }
            // TODO: handle case of unsolvable difficulty
        }
    }
}

// helper functions
/*
puts condition on the produced blockhash
thats why difficulty has to be in byte representation
*/
pub fn check_difficulty (hash: &Hash, difficulty: u128) -> bool {
    difficulty > difficulty_bytes_as_u128(&hash)
}

impl Hashable for Block {
    fn bytes (&self) -> Vec<u8> {
        let mut bytes = vec![]; // start empty and fill with block data to hash
        bytes.extend(&u32_bytes(&self.index));
        bytes.extend(&u128_bytes(&self.timestamp));
        bytes.extend(&self.prev_block_hash);
        bytes.extend(&u64_bytes(&self.nonce));
        bytes.extend(&u128_bytes(&self.difficulty));
        bytes.extend(self.transactions.as_bytes());

        bytes
    }
}

impl Debug for Block {
    fn fmt(&self, format: &mut Formatter) -> fmt::Result {
        write!(format, "from: {}\n -> Block[{}]:{} at: {} with: {} and nonce: {}",
            &hex::encode(&self.prev_block_hash),
            &self.index,
            &hex::encode(&self.block_hash),
            &self.timestamp,
            &self.transactions,
            &self.nonce,
        )
    }
}