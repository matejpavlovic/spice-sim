use sim_core::message::{Message, MessageHandler};
use sim_core::node::Node;
use crate::config::Config;
use crate::core_state::CoreState;
use crate::payload::MessagePayload;
use crate::types::{Block, ChunkID, ChunkPart, ShardID};

pub struct ChunkProducer {
    core_state: CoreState,
    shard_id: ShardID,
}

impl MessageHandler<MessagePayload> for ChunkProducer {
    fn handle_message(&mut self, node: &mut Node<MessagePayload>, msg: Message<MessagePayload>) {
        match msg.payload {
            MessagePayload::Block(block) => self.process_block(node, block),
            _ => panic!("Unknown message payload type"),
        }
    }
}

impl ChunkProducer {
    pub fn new(shard_id: ShardID, config: Config) -> Self {
        Self {
            core_state: CoreState::new(config),
            shard_id,
        }
    }

    // Apply the block; if this node is the chunk producer for this height+shard,
    // produce a chunk and disperse it to data owners.
    fn process_block(&mut self, node: &mut Node<MessagePayload>, block: Block) {
        let height = block.height;
        let block_id = block.id();
        let shard_id = self.shard_id;
        self.core_state.apply_block(block);

        if self.core_state.chunk_producer_at(height, self.shard_id) == node.id() {
            self.disperse_chunk(node, ChunkID{block_id, shard_id})
        }
    }

    // Split the chunk into parts and send each part to its assigned data owner.
    fn disperse_chunk(&mut self, node: &mut Node<MessagePayload>, chunk: ChunkID) {
        for (i, data_owner) in self.core_state.data_owner_at(chunk.block_id.height, chunk.shard_id).into_iter().enumerate() {
            let data_part = MessagePayload::ChunkPart(ChunkPart{chunk_id: chunk.clone(), index: i.try_into().unwrap()});
            node.send_message(data_part, data_owner);
        }
    }
}
