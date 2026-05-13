use std::fmt::{Display, Formatter};
use borsh::BorshSerialize;

use crate::node::NodeID;
use crate::types::chunk::ChunkID;

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
