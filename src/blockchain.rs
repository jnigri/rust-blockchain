use sha2::{Digest, Sha256};

#[derive(Clone, Debug)]
pub struct Block {
    pub hash: String,
    pub previous_hash: Option<String>,
    pub data: String,
}

pub fn add(previous_hash: Option<String>, data: &str) -> Block {
    Block {
        hash: hash(data),
        previous_hash,
        data: data.to_string(),
    }
}

fn hash(str: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(str.as_bytes());
    let result = hasher.finalize();
    result
        .iter()
        .fold(String::new(), |acc, b| format!("{}{:x?}", acc, b))
}
