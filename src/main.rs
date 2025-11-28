use std::collections::{BinaryHeap, HashMap};

use spice_sim::message::{Message, MessagePayload, MessageHandler};
use spice_sim::node::{NodeID};
use spice_sim::pingpong;

const MAX_SIMULATION_TIME: u64 = 10000;

fn main() {
    let message_queue = &mut BinaryHeap::new();

    // Create nodes.
    let mut nodes = HashMap::new();
    nodes.insert(NodeID(0), pingpong::PingPongNode::new(NodeID(0)));
    nodes.insert(NodeID(1), pingpong::PingPongNode::new(NodeID(1)));

    // Send an init message to each node.
    for node_id in nodes.keys() {
        message_queue.push(Message{
            timestamp: 0,
            source: NodeID(-1),
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
        let node = nodes.get_mut(&message.dest).unwrap();
        node.process_message(message, message_queue);
    }
}
