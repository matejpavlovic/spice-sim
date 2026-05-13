use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use borsh::BorshSerialize;

use crate::types::block::BlockID;
use crate::types::shard::ShardID;

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
