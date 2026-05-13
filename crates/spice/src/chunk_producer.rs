use sim_core::message::{Message, MessageHandler};
use sim_core::node::{Node, NodeID};
use crate::config::Config;
use crate::core_state::CoreState;
use crate::payload::MessagePayload;
use crate::types::{Block, ChunkID, ChunkPart, ShardID};

pub struct ChunkProducer {
    node: Node<MessagePayload>,
    core_state: CoreState,
    shard_id: ShardID,
}

impl MessageHandler<MessagePayload> for ChunkProducer {
    fn handle_message(&mut self, msg: Message<MessagePayload>) {
        match msg.payload {
            MessagePayload::Init => self.init(),
            MessagePayload::Block(block) => self.process_block(block),
            _ => panic!("Unknown message payload type"),
        }
    }

    fn node(&mut self) -> &mut Node<MessagePayload> {
        &mut self.node
    }
}

impl ChunkProducer {

    pub fn new(own_id: NodeID, shard_id: ShardID, config: Config) -> Self {
        Self{
            node: Node::new(own_id),
            core_state: CoreState::new(config),
            shard_id,
        }
    }

    fn init(&mut self) {
        // Nothing to initialize
    }

    fn process_block(&mut self, block: Block) {
        let height = block.height;
        let block_id = block.id();
        let shard_id = self.shard_id;
        self.core_state.apply_block(block);

        if self.core_state.chunk_producer_at(height, self.shard_id) == self.node.id() {
            self.disperse_chunk(ChunkID{block_id, shard_id})
        }

    }

    fn disperse_chunk(&mut self, chunk: ChunkID) {
        for (i, data_owner) in self.core_state.data_owner_at(chunk.block_id.height, chunk.shard_id).into_iter().enumerate() {
            let data_part = MessagePayload::ChunkPart(ChunkPart{chunk_id: chunk.clone(), index: i.try_into().unwrap()});
            self.node.send_message(data_part, data_owner);
        }
    }
}