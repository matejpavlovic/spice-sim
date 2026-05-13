use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use borsh::{BorshSerialize};
use std::ops::{Mul, Add};

use crate::node::NodeID;

#[derive(Copy, Clone, Eq, PartialEq, Hash, BorshSerialize)]
pub struct Hash([u8; 32]);

impl Display for Hash {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02x}{:02x}{:02x}{:02x}", self.0[0], self.0[1], self.0[2], self.0[3])
    }
}

impl Debug for Hash {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Hash({})", self)
    }
}

impl Ord for Hash {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for Hash {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

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

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, BorshSerialize)]
pub struct Height(pub u32);

impl Ord for Height {
    fn cmp(&self, other: &Height) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for Height {
    fn partial_cmp(&self, other: &Height) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl std::ops::Add for Height {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl From<Height> for u32 {
    fn from(value: Height) -> Self {
        value.0
    }
}

impl Mul<u32> for Height {
    type Output = u32;

    fn mul(self, rhs: u32) -> Self::Output {
        self.0 * rhs
    }
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, BorshSerialize)]
pub struct ShardID(pub u32);

impl Add<u32> for ShardID {
    type Output = u32;

    fn add(self, rhs: u32) -> Self::Output {
        self.0 + rhs
    }
}

impl Display for ShardID {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, BorshSerialize)]
pub struct ChunkID {
    pub block_id: BlockID,
    pub shard_id: ShardID,
}

impl Display for ChunkID {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Ch({}, {}, {})", self.block_id.height.0, self.block_id.hash, self.shard_id.0)
    }
}

impl Ord for ChunkID {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.block_id.cmp(&other.block_id) {
            Ordering::Equal => self.shard_id.0.cmp(&other.shard_id.0),
            ordering => ordering,
        }
    }
}

impl PartialOrd for ChunkID {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, BorshSerialize)]
pub struct ChunkPartID {
    pub chunk_id: ChunkID,
    pub index: u32,
}

impl Display for ChunkPartID {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "ChPartID({}, {})", self.chunk_id, self.index)
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, BorshSerialize)]
pub struct ChunkPart {
    pub chunk_id: ChunkID,
    pub index: u32,
}

impl Display for ChunkPart {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "ChPart({}, {})", self.chunk_id, self.index)
    }
}

impl ChunkPart {
    pub fn id(&self) -> ChunkPartID {
        ChunkPartID{chunk_id: self.chunk_id.clone(), index: self.index}
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, BorshSerialize)]
pub struct AvailCert {
    pub chunk_id: ChunkID,
    pub signers: Vec<NodeID>,
}

impl Display for AvailCert {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "AvCert({}, sigs:{})", self.chunk_id, self.signers.len())
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, BorshSerialize)]
pub struct StateCert {
    pub chunk_id: ChunkID,
    pub signers: Vec<NodeID>,
}

impl Display for StateCert {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "StCert({}, sigs:{})", self.chunk_id, self.signers.len())
    }
}
