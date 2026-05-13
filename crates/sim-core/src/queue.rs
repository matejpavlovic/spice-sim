use std::collections::BinaryHeap;
use std::hash::Hash;

use crate::message::Message;

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
