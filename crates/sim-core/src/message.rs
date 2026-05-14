use std::cmp::Ordering;
use std::fmt::{Debug, Display};
use std::hash::{Hash, Hasher};
use fxhash::FxHasher;

use crate::node::{Node, NodeID};

/// A message in flight: scheduled to arrive at `dest` at logical time `timestamp`.
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Message<P> {
    pub timestamp: u64,
    pub source: NodeID,
    pub dest: NodeID,
    pub payload: P,
}

impl<P: Display> Display for Message<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Msg({})", self.payload)
    }
}

impl<P> Message<P> {
    pub fn new(timestamp: u64, source: NodeID, dest: NodeID, payload: P) -> Self {
        Self { timestamp, source, dest, payload }
    }
}

impl<P: Hash + Eq> Ord for Message<P> {
    // Earlier timestamp first; ties broken by message hash so the order is deterministic.
    // The outer order is reversed because BinaryHeap is a max-heap and we want earliest-first.
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

/// User-implemented logic for a node. The framework calls `init` once at startup and
/// `handle_message` whenever a message destined for the node is dispatched.
pub trait MessageHandler<P>
where
    P: Hash + Eq + Display,
{
    fn handle_message(&mut self, node: &mut Node<P>, msg: Message<P>);
    fn init(&mut self, _node: &mut Node<P>) {}
}

fn hash_msg<P: Hash>(message: &Message<P>) -> u64 {
    let mut hasher = FxHasher::default();
    message.hash(&mut hasher);
    hasher.finish()
}
