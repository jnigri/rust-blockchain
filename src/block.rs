use sha2::{Digest, Sha256};

#[derive(Clone, Debug)]
pub struct BlockContent {
    pub previous_hash: Option<String>,
    pub data: String,
    pub nonce: u32,
}

#[derive(Clone, Debug)]
pub struct Block {
    pub hash: String,
    pub content: BlockContent,
}

impl Block {
    pub fn new(previous_hash: Option<String>, data: &str) -> Block {
        let (hash, content) = Block::proof_of_work(previous_hash, data, 0);
        Block { hash, content }
    }

    fn proof_of_work(
        previous_hash: Option<String>,
        data: &str,
        nonce: u32,
    ) -> (String, BlockContent) {
        let content = BlockContent {
            previous_hash: previous_hash.clone(),
            data: data.to_string(),
            nonce: nonce,
        };
        let hash = Block::hash(&format!("{:?}", content));
        if &hash[..1] == "0" {
            (hash, content)
        } else {
            Block::proof_of_work(previous_hash, data, nonce + 1)
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
}

mod tests {
    #[allow(unused_imports)]
    use super::Block;

    #[test]
    fn blockchain_can_create_new_chain() {
        let block = Block::new(None, "Hello, world!");
        assert_eq!(&block.hash[..1], "0");
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
