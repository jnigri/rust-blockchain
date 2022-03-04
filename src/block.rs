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
        let (hash, content) = Block::proof_of_work(&previous_hash, data);
        Block { hash, content }
    }

    fn proof_of_work(previous_hash: &Option<String>, data: &str) -> (String, BlockContent) {
        let mut nonce = 0u32;
        loop {
            let content = BlockContent {
                previous_hash: previous_hash.clone(),
                data: data.to_string(),
                nonce,
            };
            let hash = Block::hash(&format!("{:?}", content));
            nonce += 1;
            if Block::check_complexity(&hash) {
                return (hash, content);
            }
        }
    }

    fn check_complexity(hash: &str) -> bool {
        let n = 2;
        // check that the n first char of the hash are equal to 0
        !hash[..n].chars().any(|c| c != '0')
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
    fn it_can_create_new_block() {
        let block = Block::new(None, "Hello, world!");
        assert_eq!(&block.hash[..2], "00");
    }

    #[test]
    fn it_can_link_blocks() {
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
