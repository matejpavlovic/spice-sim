use std::collections::HashMap;
use crate::config::Config;
use crate::core_state::CoreState;
use crate::message::{Message, MessageHandler, MessagePayload};
use crate::node::{Node, NodeID};
use crate::types::{ChunkPart, ChunkPartID};

pub struct DataOwner {
    node: Node,
    core_state: CoreState,
    stored_parts: HashMap<ChunkPartID, MessagePayload>
}

impl MessageHandler for DataOwner {
    fn handle_message(&mut self, msg: Message) {
        match msg.payload {
            MessagePayload::Init => self.init(),
            MessagePayload::ChunkPart(chunk_part) => self.process_chunk_part(chunk_part),
            _ => panic!("Unknown message payload type"),
        }
    }

    fn node(&mut self) -> &mut Node {
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