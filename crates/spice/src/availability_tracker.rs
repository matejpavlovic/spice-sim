use std::collections::{BTreeMap};
use std::collections::btree_map::Entry;
use sim_core::node::NodeID;
use crate::types::{AvailCert, ChunkID, ChunkPartID};

pub struct AvailabilityTracker {
    received: BTreeMap<ChunkID, BTreeMap<u32, NodeID>>,
}

impl AvailabilityTracker {
    pub fn new() -> Self {
        Self {
            received: BTreeMap::new()
        }
    }

    /// Records a confirmation from `data_owner` for chunk_part_id.
    /// Returns `true` if newly recorded; `false` if a confirmation for that
    /// the chunk part was already present (the existing entry is kept).
    pub fn add(&mut self, data_owner: NodeID, chunk_part_id: &ChunkPartID) -> bool {
        match self.received.entry(chunk_part_id.chunk_id.clone()).or_default().entry(chunk_part_id.index) {
            Entry::Vacant(v) => {
                v.insert(data_owner);
                true
            }
            Entry::Occupied(_) => false,
        }
    }

    /// Number of distinct confirmations recorded for `chunk_id`.
    pub fn count(&self, chunk_id: &ChunkID) -> u32 {
        self.received.get(chunk_id).map_or(0, BTreeMap::len).try_into().unwrap()
    }

    /// Data owners that confirmed parts of `chunk_id`, one entry per confirmed part.
    pub fn certificate(&self, chunk_id: ChunkID) -> AvailCert {
        let Some(parts) = self.received.get(&chunk_id) else {
            return AvailCert{chunk_id, signers: Vec::new()};
        };
        AvailCert{
            chunk_id,
            signers: parts.values().cloned().collect(),
        }
    }
}

impl Default for AvailabilityTracker {
    fn default() -> Self {
        Self::new()
    }
}
