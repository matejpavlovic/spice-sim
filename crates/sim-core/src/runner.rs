use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;

use crate::message::MessageHandler;
use crate::node::NodeID;
use crate::queue::MessageQueue;

pub fn run<P>(
    nodes: &mut HashMap<NodeID, Box<dyn MessageHandler<P>>>,
    message_queue: &mut MessageQueue<P>,
    max_simulation_time: u64,
) where
    P: Hash + Eq + Display,
{
    while let Some(message) = message_queue.pop() {
        if message.timestamp > max_simulation_time {
            break;
        }

        match nodes.get_mut(&message.dest) {
            None => panic!("unknown destination {}", message.dest),
            Some(node) => node.process_message(message, message_queue),
        }
    }
}
