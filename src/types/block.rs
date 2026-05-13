use std::fmt::{Debug, Display, Formatter};
use borsh::BorshSerialize;

use crate::types::cert::{AvailCert, StateCert};
use crate::types::hash::Hash;
use crate::types::height::Height;

#[derive(Clone, Hash, Eq, PartialEq, BorshSerialize)]
pub struct Block {
    pub height: Height,
    pub parent_hash: Hash,
    pub avail_certs: Vec<AvailCert>,
    pub state_certs: Vec<StateCert>,
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Block({:03}, {}, [{}], [{}])",
               self.height.0,
               self.parent_hash,
               self.avail_certs.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(", "),
               self.state_certs.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(", "),
        )
    }
}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Block {
    pub fn genesis() -> Block {
        Block{
            height: Height(0),
            parent_hash: Hash([0u8; 32]),
            avail_certs: vec![],
            state_certs: vec![],
        }
    }

    pub fn new(height: Height, parent_hash: Hash, avail_certs: Vec<AvailCert>, state_certs: Vec<StateCert>) -> Block {
        Block{height, parent_hash, avail_certs, state_certs}
    }

    pub fn hash(&self) -> Hash {
        let mut hasher = blake3::Hasher::new();
        self.serialize(&mut hasher).unwrap();
        Hash(*(hasher.finalize().as_bytes()))
    }

    pub fn id(&self) -> BlockID {
        BlockID{
            height: self.height,
            hash: self.hash(),
        }
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd, BorshSerialize)]
pub struct BlockID {
    pub height: Height,
    pub hash: Hash,
}
//
// impl Ord for BlockID {
//     fn cmp(&self, other: &Self) -> Ordering {
//
//     }
// }
