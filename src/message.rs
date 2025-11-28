use std::collections::BinaryHeap;
use std::cmp::Ordering;
use crate::node::NodeID;

use crate::node::Node;
use crate::types;

#[derive(Debug, Eq, PartialEq)]
pub struct Message {
    pub timestamp: u64,
    pub source: NodeID,
    pub dest: NodeID,
    pub payload: MessagePayload,
}

impl Message {
    pub fn new(timestamp: u64, source: NodeID, dest: NodeID, payload: MessagePayload) -> Self {
        Self{timestamp, source, dest, payload}
    }
}

impl Ord for Message {
    fn cmp(&self, other: &Self) -> Ordering {
        self.timestamp.cmp(&other.timestamp).reverse()
    }
}

impl PartialOrd for Message {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum MessagePayload {
    Init,
    Ping(u32),
    Pong(u32),
    Block(types::Block),
}

pub trait MessageHandler {
    fn handle_message(&mut self, msg: Message);
    fn node(&mut self) -> &mut Node;
    fn process_message(&mut self, message: Message, message_output: &mut BinaryHeap<Message>) {
        self.node().pre_process_message(&message);
        self.handle_message(message);
        self.node().flush_messages(message_output);
    }
}
