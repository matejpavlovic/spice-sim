use std::collections::BinaryHeap;
use std::hash::Hash;

use crate::message::Message;

/// Priority queue of in-flight messages. Wraps `BinaryHeap` so callers don't depend on the
/// specific data structure; consumption (`pop`) is reserved for the simulator.
pub struct MessageQueue<P> {
    heap: BinaryHeap<Message<P>>,
}

impl<P> MessageQueue<P> {
    pub fn new() -> Self {
        Self { heap: BinaryHeap::new() }
    }
}

impl<P: Hash + Eq> MessageQueue<P> {
    pub fn inject(&mut self, msg: Message<P>) {
        self.heap.push(msg);
    }

    pub(crate) fn pop(&mut self) -> Option<Message<P>> {
        self.heap.pop()
    }
}

impl<P> Default for MessageQueue<P> {
    fn default() -> Self {
        Self::new()
    }
}
