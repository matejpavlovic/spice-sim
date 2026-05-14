use std::fmt::Display;

use crate::types::{Block, ChunkID, ChunkPart, ChunkPartID, StateWitness};

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum MessagePayload {
    // A newly proposed block, broadcast by a block producer.
    Block(Block),
    // One erasure-coded part of a chunk, sent from a chunk producer to a data owner for storage.
    ChunkPart(ChunkPart),
    // A data owner's attestation that it has stored a given chunk part.
    StatementChunkPartStored(ChunkPartID),
    // A replica's witness produced by executing a chunk, sent to validators.
    StateWitness(StateWitness),
    // A validator's attestation that it has verified a replica's state witness for a chunk.
    StatementStateEndorsement(ChunkID),
}

impl Display for MessagePayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessagePayload::Block(block) => write!(f, "{}", block),
            MessagePayload::ChunkPart(chunk_part) => write!(f, "{}", chunk_part),
            MessagePayload::StatementChunkPartStored(chunk_part_id) => write!(f, "ChPartStored({})", chunk_part_id),
            MessagePayload::StateWitness(witness) => write!(f, "{}", witness),
            MessagePayload::StatementStateEndorsement(chunk_id) => write!(f, "EndorseState({})", chunk_id),
        }
    }
}
