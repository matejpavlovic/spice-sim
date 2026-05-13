use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::fmt::Display;
use std::hash::{Hash, Hasher};
use fxhash::FxHasher;

use crate::node::NodeID;
use crate::node::Node;
use crate::types;
use crate::types::{ChunkPart, ChunkPartID};

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Message {
    pub timestamp: u64,
    pub source: NodeID,
    pub dest: NodeID,
    pub payload: MessagePayload,
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Msg({}: {})", self.source, self.payload)
    }
}

impl Message {
    pub fn new(timestamp: u64, source: NodeID, dest: NodeID, payload: MessagePayload) -> Self {
        Self{timestamp, source, dest, payload}
    }
}

impl Ord for Message {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.timestamp.cmp(&other.timestamp) {
            Ordering::Equal => hash_msg(self).cmp(&hash_msg(other)),
            ordering => ordering.reverse(),
        }
    }
}

impl PartialOrd for Message {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum MessagePayload {
    Init,
    Ping(u32),
    Pong(u32),
    Block(types::Block),
    ChunkPart(ChunkPart), // chunk ID and index of the data part
    StatementChunkPartStored(ChunkPartID)
}

impl Display for MessagePayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessagePayload::Block(block) => write!(f, "{}", block),
            MessagePayload::ChunkPart(chunk_part) => write!(f, "{}", chunk_part),
            MessagePayload::StatementChunkPartStored(chunk_part_id) => write!(f, "ChPartStored({})", chunk_part_id),
            _ => write!(f, "{:?}", self),
        }
    }
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

fn hash_msg(message: &Message) -> u64 {
    let mut hasher = FxHasher::default();
    message.hash(&mut hasher);
    hasher.finish()
}