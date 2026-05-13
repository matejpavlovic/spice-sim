use std::collections::{BinaryHeap, HashMap};
use spice_sim::block_producer::BlockProducer;
use spice_sim::chunk_producer::ChunkProducer;
use spice_sim::config::Config;
use spice_sim::core_state::CoreState;
use spice_sim::data_owner::DataOwner;
use spice_sim::message::{Message, MessagePayload, MessageHandler};
use spice_sim::node::{NodeID};
use spice_sim::types::ShardID;

const MAX_SIMULATION_TIME: u64 = 1000;

fn main() {
    let message_queue = &mut BinaryHeap::new();

    let config = Config::default();
    let core_state = CoreState::new(config.clone());

    // Create nodes.
    let mut nodes: HashMap<NodeID, Box<dyn MessageHandler>> = HashMap::new();
    for node_id in core_state.block_producer_ids() {
        nodes.insert(node_id, Box::new(BlockProducer::new(node_id, config.clone())));
    }
    for i in 0..config.num_shards {
        for node_id in core_state.chunk_producer_ids(ShardID(i)) {
            nodes.insert(node_id, Box::new(ChunkProducer::new(node_id, ShardID(i), config.clone())));
        }
    }
    for node_id in core_state.data_owner_ids() {
        nodes.insert(node_id, Box::new(DataOwner::new(node_id, config.clone())));
    }

    // Send an init message to each node.
    for node_id in nodes.keys() {
        message_queue.push(Message{
            timestamp: 0,
            source: NodeID::from_str("-1"),
            dest: *node_id,
            payload: MessagePayload::Init{}
        })
    }

    // Process all messages.
    while let Some(message) = message_queue.pop() {

        if message.timestamp > MAX_SIMULATION_TIME {
            break
        }

        // Using unwrap, since if the node does not exist (i.e. a message is sent to a non-existing node), we do want to panic.
        match nodes.get_mut(&message.dest) {
            None => panic!("unknown destination {}", message.dest),
            Some(node) => node.process_message(message, message_queue),
        }
    }
}
