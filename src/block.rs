use sha2::{Digest, Sha256};

#[derive(Clone, Debug)]
pub struct BlockContent {
    pub previous_hash: Option<String>,
    pub data: String,
}

#[derive(Clone, Debug)]
pub struct Block {
    pub hash: String,
    pub content: BlockContent,
}

impl Block {
    fn new(previous_hash: Option<String>, data: &str) -> Block {
        Block {
            hash: hash(data),
            content: BlockContent {
                previous_hash,
                data: data.to_string(),
            },
        }
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

mod tests {
    #[allow(unused_imports)]
    use super::Block;

    #[test]
    fn blockchain_can_create_new_chain() {
        let block = Block::new(None, "Hello, world!");
        assert_eq!(
            block.hash,
            "315f5bdb76d078c43b8ac064e4a164612b1fce77c869345bfc94c75894edd3"
        );
    }

    #[test]
    fn blockchain_can_add_new_block() {
        let first_block = Block::new(None, "first block");
        let second_block = Block::new(Some(first_block.hash.clone()), "second block");
        let third_block = Block::new(Some(second_block.hash.clone()), "third block");

        assert_eq!(
            first_block.hash,
            second_block.content.previous_hash.unwrap()
        );
        assert_eq!(
            second_block.hash,
            third_block.content.previous_hash.unwrap()
        );
    }
}
