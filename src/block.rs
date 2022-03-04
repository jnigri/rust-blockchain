use blake3;

#[derive(Clone, Debug)]
pub struct BlockContent {
    pub previous_hash: Option<[u8; 32]>,
    pub data: String,
    pub nonce: u32,
}

#[derive(Clone, Debug)]
pub struct Block {
    pub hash: [u8; 32],
    pub content: BlockContent,
}

const COMPLEXITY: usize = 2;

impl Block {
    pub fn new(previous_hash: Option<[u8; 32]>, data: &str) -> Block {
        let mut content = BlockContent {
            previous_hash,
            data: data.to_string(),
            nonce: 0u32,
        };
        // proof of work
        loop {
            let hash = Block::content_to_hash(&content);
            if Block::satisfies_proof_of_work(&hash) {
                return Block { hash, content };
            }
            content.nonce += 1;
        }
    }

    fn content_to_hash(content: &BlockContent) -> [u8; 32] {
        let content_as_string = format!("{:?}", content);
        let hash = blake3::hash(&content_as_string.as_bytes());
        hash.as_bytes().to_owned()
    }

    fn satisfies_proof_of_work(hash: &[u8; 32]) -> bool {
        let n = COMPLEXITY;
        // check that the n first char of the hash are equal to 0
        !hash[..n].iter().any(|c| *c != 0)
    }
}
