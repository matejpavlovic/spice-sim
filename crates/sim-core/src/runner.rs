use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;

use crate::message::MessageHandler;
use crate::node::{Node, NodeID};
use crate::queue::MessageQueue;

pub fn run<P>(
    nodes: &mut HashMap<NodeID, (Node<P>, Box<dyn MessageHandler<P>>)>,
    message_queue: &mut MessageQueue<P>,
    max_simulation_time: u64,
) where
    P: Hash + Eq + Display,
{
    for (node, handler) in nodes.values_mut() {
        handler.init(node);
        node.flush_messages(message_queue);
    }

    while let Some(message) = message_queue.pop() {
        if message.timestamp > max_simulation_time {
            break;
        }

        match nodes.get_mut(&message.dest) {
            None => panic!("unknown destination {}", message.dest),
            Some((node, handler)) => {
                node.pre_process_message(&message);
                handler.handle_message(node, message);
                node.flush_messages(message_queue);
            }
        }
    }
}
