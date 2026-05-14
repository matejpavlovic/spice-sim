use std::collections::BTreeMap;
use sim_core::message::{Message, MessageHandler};
use sim_core::node::{Node, NodeID};
use crate::availability_tracker::AvailabilityTracker;
use crate::config::Config;
use crate::core_state::CoreState;
use crate::endorsement_tracker::EndorsementTracker;
use crate::payload::MessagePayload;
use crate::types::{AvailCert, Block, ChunkID, ChunkPartID, Height, StateCert};

pub struct BlockProducer {
    core_state: CoreState,
    availability_tracker: AvailabilityTracker,
    endorsement_tracker: EndorsementTracker,
    pending_availability_certs: BTreeMap<ChunkID, AvailCert>,
    pending_state_certs: BTreeMap<ChunkID, StateCert>,
}

impl MessageHandler<MessagePayload> for BlockProducer {
    fn handle_message(&mut self, node: &mut Node<MessagePayload>, msg: Message<MessagePayload>) {
        match msg.payload {
            MessagePayload::Block(block) => self.process_block(node, block),
            MessagePayload::StatementChunkPartStored(chunk_part) => self.chunk_part_stored(msg.source, chunk_part),
            MessagePayload::StatementStateEndorsement(chunk_id) => self.state_endorsed(msg.source, chunk_id),
            _ => panic!("Unknown message payload type"),
        }
    }
    // If this node is the producer assigned to height 0, broadcast the genesis block.
    fn init(&mut self, node: &mut Node<MessagePayload>) {
        if self.core_state.block_producer_at(Height(0)) == node.id() {
            self.broadcast_block(node, Block::genesis())
        }
    }
}

impl BlockProducer {
    pub fn new(config: Config) -> Self {
        Self {
            core_state: CoreState::new(config),
            availability_tracker: AvailabilityTracker::new(),
            endorsement_tracker: EndorsementTracker::new(),
            pending_availability_certs: BTreeMap::new(),
            pending_state_certs: BTreeMap::new(),
        }
    }

    // Send the block to every block producer, every chunk producer, every replica across all
    // shards, and every validator.
    fn broadcast_block(&mut self, node: &mut Node<MessagePayload>, block: Block) {
        let mut destinations = vec![];
        destinations.append(&mut self.core_state.block_producer_ids());
        destinations.append(&mut self.core_state.chunk_producer_ids_all_shards());
        destinations.append(&mut self.core_state.replica_ids_all_shards());
        destinations.append(&mut self.core_state.validator_ids());

        for dest in destinations {
            node.send_message(MessagePayload::Block(block.clone()), dest)
        }
    }

    // Apply the received block to local state; if this node is the producer for the next height,
    // build and broadcast the next block carrying any pending availability and state certificates.
    fn process_block(&mut self, node: &mut Node<MessagePayload>, block: Block) {
        let height = block.height;
        let hash = block.hash();

        for avail_cert in &block.avail_certs {
            self.pending_availability_certs.remove(&avail_cert.chunk_id);
        }
        for state_cert in &block.state_certs {
            self.pending_state_certs.remove(&state_cert.chunk_id);
        }

        self.core_state.apply_block(block);

        if self.core_state.block_producer_at(height + Height(1)) == node.id() {
            self.broadcast_block(node, Block::new(
                height + Height(1),
                hash,
                self.pending_availability_certs.values().cloned().collect(),
                self.pending_state_certs.values().cloned().collect(),
            ));
        }
    }

    // Record a data owner's attestation; once the availability quorum is met for a chunk, create an
    // availability certificate that will be included in a future block.
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

    // Record a validator's state endorsement; once the endorsement quorum is met, create a state
    // certificate that will be included in a future block.
    fn state_endorsed(&mut self, validator: NodeID, chunk_id: ChunkID) {
        self.endorsement_tracker.add(validator, &chunk_id);

        if self.endorsement_tracker.count(&chunk_id) == self.core_state.config().endorsement_quorum {
            self.pending_state_certs.insert(chunk_id.clone(), self.endorsement_tracker.certificate(chunk_id));
        }
    }
}
