use std::collections::HashMap;
use sim_core::message::{Message, MessageHandler};
use sim_core::node::Node;
use crate::config::Config;
use crate::core_state::CoreState;
use crate::payload::MessagePayload;
use crate::types::{ChunkPart, ChunkPartID};

pub struct DataOwner {
    core_state: CoreState,
    stored_parts: HashMap<ChunkPartID, MessagePayload>
}

impl MessageHandler<MessagePayload> for DataOwner {
    fn handle_message(&mut self, node: &mut Node<MessagePayload>, msg: Message<MessagePayload>) {
        match msg.payload {
            MessagePayload::ChunkPart(chunk_part) => self.process_chunk_part(node, chunk_part),
            _ => panic!("Unknown message payload type"),
        }
    }
}

impl DataOwner {
    pub fn new(config: Config) -> Self {
        Self {
            core_state: CoreState::new(config),
            stored_parts: HashMap::new(),
        }
    }

    fn process_chunk_part(&mut self, node: &mut Node<MessagePayload>, chunk_part: ChunkPart) {
        self.submit_statement_chunk_part_stored(node, chunk_part.id());
        self.stored_parts.insert(chunk_part.id(), MessagePayload::ChunkPart(chunk_part));
    }

    fn submit_statement_chunk_part_stored(&mut self, node: &mut Node<MessagePayload>, chunk_part_id: ChunkPartID) {
        for block_producer in self.core_state.block_producer_ids() {
            node.send_message(MessagePayload::StatementChunkPartStored(chunk_part_id.clone()), block_producer);
        }
    }
}
