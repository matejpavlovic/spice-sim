use blake3::Hash;

#[derive(Debug, Eq, PartialEq)]
pub struct Block {
    height: u64,
    parent_hash: Hash
}