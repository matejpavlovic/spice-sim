use std::collections::BTreeMap;

use sim_core::node::NodeID;

use crate::types::{ChunkID, StateCert};

/// Tracks distinct state endorsements received per chunk so a block producer can decide
/// when enough validators have endorsed a chunk's execution to issue a `StateCert`. Signers
/// are kept in insertion order, which is deterministic given the simulator's deterministic
/// message dispatch — every block producer observes the same sequence of endorsements.
pub struct EndorsementTracker {
    received: BTreeMap<ChunkID, Vec<NodeID>>,
}

impl EndorsementTracker {
    pub fn new() -> Self {
        Self { received: BTreeMap::new() }
    }

    /// Records an endorsement from `validator` for `chunk_id`. Returns `true` if newly recorded;
    /// `false` if this validator already endorsed this chunk.
    pub fn add(&mut self, validator: NodeID, chunk_id: &ChunkID) -> bool {
        let signers = self.received.entry(chunk_id.clone()).or_default();
        if signers.contains(&validator) {
            false
        } else {
            signers.push(validator);
            true
        }
    }

    /// Number of distinct endorsements recorded for `chunk_id`.
    pub fn count(&self, chunk_id: &ChunkID) -> u32 {
        self.received.get(chunk_id).map_or(0, Vec::len).try_into().unwrap()
    }

    /// Build a state certificate from the recorded endorsements.
    pub fn certificate(&self, chunk_id: ChunkID) -> StateCert {
        let signers = self.received.get(&chunk_id).cloned().unwrap_or_default();
        StateCert { chunk_id, signers }
    }
}

impl Default for EndorsementTracker {
    fn default() -> Self {
        Self::new()
    }
}
