use std::collections::HashMap;
use sim_core::message::MessageHandler;
use sim_core::node::{Node, NodeID};
use sim_core::queue::MessageQueue;
use sim_core::runner::run;
use spice::block_producer::BlockProducer;
use spice::chunk_producer::ChunkProducer;
use spice::config::Config;
use spice::core_state::CoreState;
use spice::data_owner::DataOwner;
use spice::payload::MessagePayload;
use spice::types::ShardID;

const MAX_SIMULATION_TIME: u64 = 1000;

fn main() {
    let mut message_queue = MessageQueue::new();

    let config = Config::default();
    let core_state = CoreState::new(config.clone());

    // Create nodes.
    let mut nodes: HashMap<NodeID, (Node<MessagePayload>, Box<dyn MessageHandler<MessagePayload>>)> = HashMap::new();
    for node_id in core_state.block_producer_ids() {
        nodes.insert(node_id, (Node::new(node_id), Box::new(BlockProducer::new(config.clone()))));
    }
    for i in 0..config.num_shards {
        for node_id in core_state.chunk_producer_ids(ShardID(i)) {
            nodes.insert(node_id, (Node::new(node_id), Box::new(ChunkProducer::new(node_id, ShardID(i), config.clone()))));
        }
    }
    for node_id in core_state.data_owner_ids() {
        nodes.insert(node_id, (Node::new(node_id), Box::new(DataOwner::new(config.clone()))));
    }

    run(&mut nodes, &mut message_queue, MAX_SIMULATION_TIME);
}
