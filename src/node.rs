use std::cmp::max;
use std::collections::BinaryHeap;
use std::fmt::Display;
use crate::message::{Message, MessagePayload};
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct NodeID(pub i32);

impl Display for NodeID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

const MESSAGE_DELAY: u64 = 100;

pub struct Node {
    own_id: NodeID,
    local_time: u64,
    msg_buffer: Vec<Message>,
}

impl Node {
    pub fn new(own_id: NodeID) -> Self {
        Self { own_id, local_time: 0, msg_buffer: Vec::new() }
    }

    pub fn id(&self) -> NodeID {
        self.own_id
    }

    pub fn time(&self) -> u64 {
        self.local_time
    }

    pub fn pre_process_message(&mut self, msg: &Message) {
        self.local_time = max(self.local_time, msg.timestamp);
        println!("{:>5} {:}: {:?}", self.local_time, self.own_id, msg);
    }

    pub fn advance_time(&mut self, step: u64) {
        self.local_time += step;
    }

    pub fn send_message(&mut self, payload: MessagePayload, dest: NodeID) {
        self.msg_buffer.push(Message::new(self.local_time + MESSAGE_DELAY, self.own_id, dest, payload));
    }

    pub fn flush_messages(&mut self, msg_queue: &mut BinaryHeap<Message>) {
        for msg in self.msg_buffer.drain(..) {
            msg_queue.push(msg);
        }
    }
}

#[macro_export]
macro_rules! nlog {
    ($node:expr, $($arg:tt)*) => {{
        let node = &$node;
        println!(
            "[{:>5} {:?}] {}",
            node.time(),
            node.id(),
            format_args!($($arg)*),
        );
    }};
}
