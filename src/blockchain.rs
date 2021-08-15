use super::*;

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new (genesis_block: Block) -> Self {
        Blockchain {
            blocks: vec![genesis_block],
        }
    }

    pub fn verify_chain (&self) -> bool {
        for (idx, block) in self.blocks.iter().enumerate() {
            // add all the necessary verifications (soft verify,for learning purposes)
            if block.index != idx as u32 {
                println!("Index mismatch {} unequal {}", &block.index, &idx );
                return false;
            } else if !block::check_difficulty(&block.hash(), block.difficulty){
                // TODO dont just trust the difficulty specified in the block, create hard verification
                println!("Difficulty constraints not fulfilled");
                return false;
            // check previous block_hash
            } else if idx != 0 {
                // non-genesis block
                // additionally check timestep increase (simple version not sufficiant for real blockchain [node clocks out of sync. problem etc.])
                let prev_block = &self.blocks[idx-1];
                if block.timestamp <= prev_block.timestamp {
                    println!("Timestamps out of order");
                    return false;
                } else if block.prev_block_hash != prev_block.block_hash {
                    println!("Referred prev_block_hash missmatched");
                    return false;
                }
            } else { // idx == 0
                // genesis block, prev_block_hash is hardcoded
                if block.prev_block_hash != vec![0; 32] {
                    println!("Genesis block prev_block_hash is invalid");
                    return false;
                }
            }
        }
        return true;
    }
}