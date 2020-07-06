use super::*;

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn verify(&self) -> bool {
        for (i, block) in self.blocks.iter().enumerate() {
            if block.index != i as u32 {
                println!("Index missmatch {} does not equal {}",
                    &block.index,
                    &i,
                );
                return false;
            } else if !block::check_difficulty(&block.hash(), block.difficulty) {
                println!("Difficulty failed");
                return false;
            } else if i != 0 {
                let prev_block = &self.blocks[i-1];
                if block.timestamp <= prev_block.timestamp {
                    println!("Time did not increase");
                    return false;
                } else if block.prev_block_hash != prev_block.hash {
                    println!("Hash missmatch");
                    return false;
                }
            } else {
                if block.prev_block_hash != vec![0; 32] {
                    println!("Genesis block previous block hash is invalid");
                    return false;
                }
            }
        }
        true
    }
}
