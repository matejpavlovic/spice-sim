use sim_core::message::{Message, MessageHandler};
use sim_core::node::{Node, NodeID};

use crate::config::Config;
use crate::core_state::CoreState;
use crate::endorsement_tracker::EndorsementTracker;
use crate::payload::MessagePayload;
use crate::types::{Block, ChunkID, Height, ShardID, StateWitness};

/// Observes the chain, executes its shard's chunks once they become available, and sends a
/// state witness for each executed chunk to the sampled validators. Also tracks the state
/// endorsements that validators send out, so the replica can later make progress decisions
/// independently of the block producers and the certificates included on chain.
pub struct Replica {
    core_state: CoreState,
    shard_id: ShardID,
    next_height: Height,
    endorsement_tracker: EndorsementTracker,
}

impl MessageHandler<MessagePayload> for Replica {
    fn handle_message(&mut self, node: &mut Node<MessagePayload>, msg: Message<MessagePayload>) {
        match msg.payload {
            MessagePayload::Block(block) => self.process_block(node, block),
            MessagePayload::StatementStateEndorsement(chunk_id) => self.state_endorsed(msg.source, chunk_id),
            _ => panic!("Unknown message payload type"),
        }
    }
}

impl Replica {
    pub fn new(shard_id: ShardID, config: Config) -> Self {
        Self {
            core_state: CoreState::new(config),
            shard_id,
            next_height: Height(0),
            endorsement_tracker: EndorsementTracker::new(),
        }
    }

    // Record an endorsement from `validator`. Quorum-driven progress will hang off this later.
    fn state_endorsed(&mut self, validator: NodeID, chunk_id: ChunkID) {
        self.endorsement_tracker.add(validator, &chunk_id);
    }

    fn process_block(&mut self, node: &mut Node<MessagePayload>, block: Block) {
        self.core_state.apply_block(block);
        self.execute_ready(node);
    }

    // Walk forward through the chain, executing the chunk at `next_height` for our shard as
    // long as both the block exists and the chunk has been certified as available. Stops at
    // the first gap; later blocks may close it and the next call resumes from where we left off.
    fn execute_ready(&mut self, node: &mut Node<MessagePayload>) {
        loop {
            let block_id = match self.core_state.canonical_block_at(self.next_height) {
                Some(block) => block.id(),
                None => break,
            };
            let chunk_id = ChunkID { block_id, shard_id: self.shard_id };
            if !self.core_state.chunk_available(&chunk_id) {
                break;
            }
            self.execute_chunk(node, chunk_id);
            self.next_height = self.next_height + Height(1);
        }
    }

    // "Execute" the chunk (a no-op for now) and dispatch a state witness to every validator
    // sampled for this (height, shard).
    fn execute_chunk(&mut self, node: &mut Node<MessagePayload>, chunk_id: ChunkID) {
        let witness = StateWitness { chunk_id: chunk_id.clone() };
        for validator in self.core_state.validators_at(chunk_id.block_id.height, chunk_id.shard_id) {
            node.send_message(MessagePayload::StateWitness(witness.clone()), validator);
        }
    }
}
