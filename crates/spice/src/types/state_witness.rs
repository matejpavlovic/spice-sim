use std::fmt::{Display, Formatter};
use borsh::BorshSerialize;

use crate::types::chunk::ChunkID;

#[derive(Clone, Debug, Hash, Eq, PartialEq, BorshSerialize)]
pub struct StateWitness {
    pub chunk_id: ChunkID,
}

impl Display for StateWitness {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "StWit({})", self.chunk_id)
    }
}
