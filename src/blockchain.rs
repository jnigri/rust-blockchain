use sha2::{Digest, Sha256, Sha512};

#[derive(Clone, Debug)]
pub struct Block {
    hash: String,
    previous_block_hash: Option<String>,
    data: String,
}

#[derive(Clone, Debug)]
pub struct Blockchain {
    first: Block,
    last: Block,
}

impl Blockchain {
    pub fn new(genesis_data: &str) -> Blockchain {
        let genesis_block = Block {
            hash: Blockchain::hash(&genesis_data),
            previous_block_hash: None,
            data: genesis_data.to_string(),
        };
        Blockchain {
            first: genesis_block.clone(),
            last: genesis_block,
        }
    }

    // fn add(&self, data: String) -> Block {}

    pub fn hash(str: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(str.as_bytes());
        let result = hasher.finalize();
        result
            .iter()
            .fold(String::new(), |acc, b| format!("{}{:x?}", acc, b))
    }
}
