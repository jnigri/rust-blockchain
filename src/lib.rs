pub mod block;

#[cfg(test)]
mod tests {
    use crate::block::Block;

    #[test]
    fn it_can_create_new_block() {
        let block = Block::new(None, "Hello, world!");
        assert_eq!(&block.hash[..2], [0u8, 0u8]);
    }

    #[test]
    fn it_can_link_blocks() {
        let first_block = Block::new(None, "first block");
        let second_block = Block::new(Some(first_block.hash), "second block");
        let third_block = Block::new(Some(second_block.hash), "third block");

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
