use std::fmt::Display;

use crate::types::{Block, ChunkPart, ChunkPartID};

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum MessagePayload {
    Init,
    Block(Block),
    ChunkPart(ChunkPart), // chunk ID and index of the data part
    StatementChunkPartStored(ChunkPartID)
}

impl Display for MessagePayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessagePayload::Block(block) => write!(f, "{}", block),
            MessagePayload::ChunkPart(chunk_part) => write!(f, "{}", chunk_part),
            MessagePayload::StatementChunkPartStored(chunk_part_id) => write!(f, "ChPartStored({})", chunk_part_id),
            _ => write!(f, "{:?}", self),
        }
    }
}
