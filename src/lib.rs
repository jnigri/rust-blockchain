mod blockchain;
pub use blockchain::*;

#[cfg(test)]
mod tests {
    use crate::blockchain;
    #[test]
    fn blockchain_can_create_new_chain() {
        let block = blockchain::add(None, "Hello, world!");
        assert_eq!(
            block.hash,
            "315f5bdb76d078c43b8ac064e4a164612b1fce77c869345bfc94c75894edd3"
        );
    }

    #[test]
    fn blockchain_can_add_new_block() {
        let first_block = blockchain::add(None, "first block");
        let second_block = blockchain::add(Some(first_block.hash.clone()), "second block");
        let third_block = blockchain::add(Some(second_block.hash.clone()), "third block");

        assert_eq!(first_block.hash, second_block.previous_hash.unwrap());
        assert_eq!(second_block.hash, third_block.previous_hash.unwrap());
    }
}
