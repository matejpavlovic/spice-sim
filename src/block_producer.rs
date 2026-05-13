use std::collections::BTreeMap;
use crate::availability_tracker::AvailabilityTracker;
use crate::config::Config;
use crate::core_state::CoreState;
use crate::message::{Message, MessageHandler, MessagePayload};
use crate::node::{Node, NodeID};
use crate::types::{AvailCert, Block, ChunkID, ChunkPartID, Height};

pub struct BlockProducer {
    node: Node,
    core_state: CoreState,
    availability_tracker: AvailabilityTracker,
    pending_availability_certs: BTreeMap<ChunkID, AvailCert>
}

impl MessageHandler for BlockProducer {
    fn handle_message(&mut self, msg: Message) {
        match msg.payload {
            MessagePayload::Init => self.init(),
            MessagePayload::Block(block) => self.process_block(block),
            MessagePayload::StatementChunkPartStored(chunk_part) => self.chunk_part_stored(msg.source, chunk_part),
            _ => panic!("Unknown message payload type"),
        }
    }
    fn node(&mut self) -> &mut Node {
        &mut self.node
    }
}

impl BlockProducer {
    pub fn new(own_id: NodeID, config: Config) -> Self {
        Self{
            node: Node::new(own_id),
            core_state: CoreState::new(config),
            availability_tracker: AvailabilityTracker::new(),
            pending_availability_certs: BTreeMap::new(),
        }
    }

    fn broadcast_block(&mut self, block: Block) {
        let mut destinations = vec![];
        destinations.append(&mut self.core_state.block_producer_ids());
        destinations.append(&mut self.core_state.chunk_producer_ids_all_shards());

        for node in destinations {
            self.node.send_message(MessagePayload::Block(block.clone()), node)
        }
    }

    fn init(&mut self) {
        if self.core_state.block_producer_at(Height(0)) == self.node.id() {
            self.broadcast_block(Block::genesis())
        }
    }

    fn process_block(&mut self, block: Block) {
        let height = block.height;
        let hash = block.hash();

        for avail_cert in &block.avail_certs {
            self.pending_availability_certs.remove(&avail_cert.chunk_id);
        }

        self.core_state.apply_block(block);

        if self.core_state.block_producer_at(height + Height(1)) == self.node.id() {
            self.broadcast_block(Block::new(height + Height(1), hash, self.pending_availability_certs.values().cloned().collect(), vec![]));
        }
    }

    fn chunk_part_stored(&mut self, data_owner: NodeID, chunk_part_id: ChunkPartID) {
        // If we want actual fault tolerance, we need to check here if the part owner is really assigned to this part.
        // Below, we then need to check if a sufficient number of distinct owners confirmed a sufficient number of
        // different chunk parts to guarantee reconstruction despite failures.
        // The analogous needs to be done for endorsements (except that they don't have parts).

        self.availability_tracker.add(data_owner, &chunk_part_id);

        if self.availability_tracker.count(&chunk_part_id.chunk_id) == self.core_state.config().availability_quorum {
            self.pending_availability_certs.insert(chunk_part_id.chunk_id.clone(), self.availability_tracker.certificate(chunk_part_id.chunk_id.clone()));
        }
    }
}