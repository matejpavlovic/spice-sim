use std::fmt::Display;

use crate::types::{Block, ChunkPart, ChunkPartID};

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum MessagePayload {
    // A newly proposed block, broadcast by a block producer.
    Block(Block),
    // One erasure-coded part of a chunk, sent from a chunk producer to a data owner for storage.
    ChunkPart(ChunkPart),
    // A data owner's attestation that it has stored a given chunk part.
    StatementChunkPartStored(ChunkPartID)
}

impl Display for MessagePayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessagePayload::Block(block) => write!(f, "{}", block),
            MessagePayload::ChunkPart(chunk_part) => write!(f, "{}", chunk_part),
            MessagePayload::StatementChunkPartStored(chunk_part_id) => write!(f, "ChPartStored({})", chunk_part_id),
        }
    }
}
