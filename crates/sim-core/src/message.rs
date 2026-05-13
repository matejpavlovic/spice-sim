use std::cmp::Ordering;
use std::fmt::{Debug, Display};
use std::hash::{Hash, Hasher};
use fxhash::FxHasher;

use crate::node::{Node, NodeID};
use crate::queue::MessageQueue;

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Message<P> {
    pub timestamp: u64,
    pub source: NodeID,
    pub dest: NodeID,
    pub payload: P,
}

impl<P: Display> Display for Message<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Msg({}: {})", self.source, self.payload)
    }
}

impl<P> Message<P> {
    pub fn new(timestamp: u64, source: NodeID, dest: NodeID, payload: P) -> Self {
        Self { timestamp, source, dest, payload }
    }
}

impl<P: Hash + Eq> Ord for Message<P> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.timestamp.cmp(&other.timestamp) {
            Ordering::Equal => hash_msg(self).cmp(&hash_msg(other)),
            ordering => ordering.reverse(),
        }
    }
}

impl<P: Hash + Eq> PartialOrd for Message<P> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub trait MessageHandler<P>
where
    P: Hash + Eq + Display,
{
    fn handle_message(&mut self, msg: Message<P>);
    fn node(&mut self) -> &mut Node<P>;
    fn process_message(&mut self, message: Message<P>, message_output: &mut MessageQueue<P>) {
        self.node().pre_process_message(&message);
        self.handle_message(message);
        self.node().flush_messages(message_output);
    }
}

fn hash_msg<P: Hash>(message: &Message<P>) -> u64 {
    let mut hasher = FxHasher::default();
    message.hash(&mut hasher);
    hasher.finish()
}
