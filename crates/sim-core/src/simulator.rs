use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;

use crate::message::MessageHandler;
use crate::node::{Node, NodeID};
use crate::queue::MessageQueue;

/// Owns every node's framework state and the shared message queue, and drives the simulation.
pub struct Simulator<P> {
    nodes: HashMap<NodeID, (Node<P>, Box<dyn MessageHandler<P>>)>,
    queue: MessageQueue<P>,
}

impl<P> Simulator<P>
where
    P: Hash + Eq + Display,
{
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            queue: MessageQueue::new(),
        }
    }

    pub fn register<H: MessageHandler<P> + 'static>(&mut self, id: NodeID, handler: H) {
        self.nodes.insert(id, (Node::new(id), Box::new(handler)));
    }

    pub fn run(&mut self, max_simulation_time: u64) {
        // Bootstrap: let every handler run its init hook and flush any messages it produced.
        for (node, handler) in self.nodes.values_mut() {
            handler.init(node);
            node.flush_messages(&mut self.queue);
        }

        // Dispatch: drain the queue in (timestamp, hash) order until empty or past the deadline.
        while let Some(message) = self.queue.pop() {
            if message.timestamp > max_simulation_time {
                break;
            }

            match self.nodes.get_mut(&message.dest) {
                None => panic!("unknown destination {}", message.dest),
                Some((node, handler)) => {
                    node.pre_process_message(&message);
                    handler.handle_message(node, message);
                    node.flush_messages(&mut self.queue);
                }
            }
        }
    }
}

impl<P> Default for Simulator<P>
where
    P: Hash + Eq + Display,
{
    fn default() -> Self {
        Self::new()
    }
}
