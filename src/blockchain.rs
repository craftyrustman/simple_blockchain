use crate::block::Block;
use crate::block::calculate_hash;

use std::time::{ SystemTime, UNIX_EPOCH};
use std::process;

#[derive(Debug)]
pub struct BlockChain{
   pub blocks: Vec<Block>
}

impl BlockChain {
    pub fn new() -> BlockChain {
        BlockChain { blocks: vec![]}
    }
    pub fn genesis(&mut self) {
        let genesis_block = Block{
            index: 0,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            data: String::from("genesis block"),
            previous_hash: String::from("0"),
            hash: String::from("0"),
            nonce: 0
        };
        self.blocks.push(genesis_block)
    }

    pub fn add_block(&mut self, block: Block){
        if self.is_block_valid(&block) {
            self.blocks.push(block)
        } else {
            println!("invalid block");
            process::exit(1)
        }
    }

    fn is_block_valid(&self, block: &Block) -> bool {
        let last_block = self.blocks.last().unwrap();
        last_block.index + 1 == block.index && last_block.hash == block.previous_hash && block.
        hash == calculate_hash(block.index, block.timestamp, &block.data, &block.previous_hash, block.nonce) 
    }
}
