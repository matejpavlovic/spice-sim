use sim_core::message::{Message, MessageHandler};
use sim_core::node::Node;

use crate::config::Config;
use crate::core_state::CoreState;
use crate::payload::MessagePayload;
use crate::types::{Block, ChunkID, Height, ShardID, StateWitness};

/// Observes the chain, executes its shard's chunks once they become available, and ships a
/// state witness for each executed chunk to the sampled validators.
pub struct Replica {
    core_state: CoreState,
    shard_id: ShardID,
    next_height: Height,
}

impl MessageHandler<MessagePayload> for Replica {
    fn handle_message(&mut self, node: &mut Node<MessagePayload>, msg: Message<MessagePayload>) {
        match msg.payload {
            MessagePayload::Block(block) => self.process_block(node, block),
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
        }
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
