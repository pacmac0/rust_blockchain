use super::*;
use std::collections::HashSet;

#[derive(Debug)]
pub enum BlockValidationError {
    MismatchedIndex,
    InvalidHash,
    AchronologicalTimestamp,
    MismatchedPreviousHash,
    InvalidGenesisBlockFormat,
    InvalidInput,
    InsufficientInputValue,
    InvalidCoinbaseTransaction,
}

pub struct Blockchain {
    pub blocks: Vec<Block>,
    unspent_outputs: HashSet<Hash>,
}

impl Blockchain {
    pub fn new () -> Self {
        Blockchain {
            blocks: vec![],
            unspent_outputs: HashSet::new(),
        }
    }

    pub fn update_add_block (&mut self, block: Block) -> Result<(), BlockValidationError> {
        let idx = self.blocks.len();
        
        
        // add all the necessary verifications (soft verify, for learning purposes)
        if block.index != idx as u32 {
            return Err(BlockValidationError::MismatchedIndex);
        } else if !block::check_difficulty(&block.hash(), block.difficulty){
            // TODO dont just trust the difficulty specified in the block, create hard verification
            return Err(BlockValidationError::InvalidHash);
        // check previous block_hash
        } else if idx != 0 {
            // non-genesis block
            // additionally check timestep increase (simple version not sufficiant for real blockchain [node clocks out of sync. problem etc.])
            let prev_block = &self.blocks[idx-1];
            if block.timestamp <= prev_block.timestamp {
                return Err(BlockValidationError::AchronologicalTimestamp);
            } else if block.prev_block_hash != prev_block.block_hash {
                return Err(BlockValidationError::MismatchedPreviousHash);
            }
        } else { // idx == 0
            // genesis block, prev_block_hash is hardcoded
            if block.prev_block_hash != vec![0; 32] {
                return Err(BlockValidationError::InvalidGenesisBlockFormat);
            }
        }

        if let Some((coinbase, transactions)) = block.transactions.split_first() {
            if !coinbase.is_coinbase() {
                return Err(BlockValidationError::InvalidCoinbaseTransaction);
            }

            let mut block_spent: HashSet<Hash> = HashSet::new();
            let mut block_created: HashSet<Hash> = HashSet::new();
            let mut total_fee = 0;

            for transaction in transactions {
                let input_hashes = transaction.input_hashes();

                if !(&input_hashes - &self.unspent_outputs).is_empty() ||
                   !(&input_hashes & &block_spent).is_empty()
                {
                    return Err(BlockValidationError::InvalidInput);
                }
                let input_value = transaction.input_value();
                let output_value = transaction.output_value();

                if output_value > input_value {
                    return Err(BlockValidationError::InsufficientInputValue);
                }

                let fee = input_value - output_value;
                total_fee += fee;

                block_spent.extend(input_hashes);
                block_created.extend(transaction.output_hashes());
            }

            if coinbase.output_value() < total_fee { // TODO still allows for adding any amount of coins to miner
                return Err(BlockValidationError::InvalidCoinbaseTransaction);
            } else {
                block_created.extend(coinbase.output_hashes());
            }

            self.unspent_outputs.retain(|output| !block_spent.contains(output));
            self.unspent_outputs.extend(block_created);
        }

        self.blocks.push(block);
        
        Ok(())
    }
}