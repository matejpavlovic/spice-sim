use std::cmp::max;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use borsh::BorshSerialize;

use crate::message::Message;
use crate::queue::MessageQueue;

#[derive(Copy, Clone, Eq, PartialEq, Hash, BorshSerialize)]
pub struct NodeID(pub [u8; 32]);

impl Display for NodeID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let end = self.0.iter().position(|&b| b == 0).unwrap_or(self.0.len());
        let s = String::from_utf8_lossy(&self.0[..end]);
        f.write_str(&s)
    }
}

impl Debug for NodeID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

impl NodeID {
    pub fn from_str(s: &str) -> NodeID {
        if s.as_bytes().len() > 32 {
            panic!("NodeID ({}) length exceeded 32 bytes", s);
        }
        let bytes = s.as_bytes();
        let mut id = [0u8; 32];
        id[..bytes.len()].copy_from_slice(bytes);
        NodeID(id)
    }
}

const MESSAGE_DELAY: u64 = 100;

pub struct Node<P> {
    own_id: NodeID,
    local_time: u64,
    msg_buffer: Vec<Message<P>>,
}

impl<P> Node<P> {
    pub fn new(own_id: NodeID) -> Self {
        Self { own_id, local_time: 0, msg_buffer: Vec::new() }
    }

    pub fn id(&self) -> NodeID {
        self.own_id
    }

    pub fn time(&self) -> u64 {
        self.local_time
    }

    pub fn log(&self, msg: &str) {
        println!("{:>5} {:}: {}", self.local_time, self.own_id, msg);
    }

    pub fn advance_time(&mut self, step: u64) {
        self.local_time += step;
    }
}

impl<P: Display> Node<P> {
    pub fn pre_process_message(&mut self, msg: &Message<P>) {
        self.local_time = max(self.local_time, msg.timestamp);
        println!("RCV {:>5} {:>20}: {}", self.local_time, self.own_id, msg);
    }

    pub fn send_message(&mut self, payload: P, dest: NodeID) {
        let msg = Message::new(self.local_time + MESSAGE_DELAY, self.own_id, dest, payload);
        println!("SND {:>5} {:>20} -> {:>20}: {}", self.local_time, self.own_id, dest, msg);
        self.msg_buffer.push(msg);
    }
}

impl<P: Hash + Eq> Node<P> {
    pub fn flush_messages(&mut self, msg_queue: &mut MessageQueue<P>) {
        for msg in self.msg_buffer.drain(..) {
            msg_queue.inject(msg);
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
