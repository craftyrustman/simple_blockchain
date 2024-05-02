use std::{collections::hash_map::DefaultHasher, hash::{self, Hash, Hasher}, process, time::{SystemTime, UNIX_EPOCH}};

static DIFFICULTY: &str = "100";

#[derive(Debug)]
pub struct Block{
    pub index: usize,
    pub timestamp: u64,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: usize
}

impl Block {
    pub fn new(index: usize, data: String, previous_block: &Block) -> Block{
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let previous_hash = previous_block.hash.clone();
        let (nonce, hash) = mine_block(index, timestamp, &data, &previous_hash);
        Block { index, timestamp, data, previous_hash, hash, nonce }
    }
}

fn mine_block(index: usize, timestamp: u64, data: &String, previous_hash: &String) -> (usize, String){
    let mut nonce = 0;
    loop {
        if nonce == 1000 {
            println!("After 1000 times, mined 0 block");
            process::exit(1)
        }
        let hash = calculate_hash(index, timestamp, data, previous_hash, nonce);
        if hash.starts_with(DIFFICULTY){
            return (nonce, hash);
        }
        nonce += 1;
    }
}

pub fn calculate_hash(index: usize, timestamp: u64, data: &String, previous_hash: &String, nonce: usize) -> String {
    let mut hasher = DefaultHasher::new();
    let input = format!("{}{}{}{}{}", index, timestamp, data, previous_hash, nonce);
    input.hash(&mut hasher);
    hasher.finish().to_string()
}