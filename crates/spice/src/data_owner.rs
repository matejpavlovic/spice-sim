use std::collections::HashMap;
use sim_core::message::{Message, MessageHandler};
use sim_core::node::{Node, NodeID};
use crate::config::Config;
use crate::core_state::CoreState;
use crate::payload::MessagePayload;
use crate::types::{ChunkPart, ChunkPartID};

pub struct DataOwner {
    node: Node<MessagePayload>,
    core_state: CoreState,
    stored_parts: HashMap<ChunkPartID, MessagePayload>
}

impl MessageHandler<MessagePayload> for DataOwner {
    fn handle_message(&mut self, msg: Message<MessagePayload>) {
        match msg.payload {
            MessagePayload::Init => self.init(),
            MessagePayload::ChunkPart(chunk_part) => self.process_chunk_part(chunk_part),
            _ => panic!("Unknown message payload type"),
        }
    }

    fn node(&mut self) -> &mut Node<MessagePayload> {
        &mut self.node
    }
}

impl DataOwner {
    pub fn new(own_id: NodeID, config: Config) -> Self {
        Self{
            node: Node::new(own_id),
            core_state: CoreState::new(config),
            stored_parts: HashMap::new(),
        }
    }

    pub fn init(&mut self) {
    }

    fn process_chunk_part(&mut self, chunk_part: ChunkPart) {
        self.submit_statement_chunk_part_stored(chunk_part.id());
        self.stored_parts.insert(chunk_part.id(), MessagePayload::ChunkPart(chunk_part));
    }

    fn submit_statement_chunk_part_stored(&mut self, chunk_part_id: ChunkPartID) {
        for block_producer in self.core_state.block_producer_ids() {
            self.node.send_message(MessagePayload::StatementChunkPartStored(chunk_part_id.clone()), block_producer);
        }
    }
}