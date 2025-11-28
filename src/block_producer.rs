use crate::message::{Message, MessageHandler, MessagePayload};
use crate::node::{Node, NodeID};
use crate::types::Block;

pub struct BlockProducer {
    node: Node,
}

impl MessageHandler for BlockProducer {
    fn handle_message(&mut self, msg: Message) {
        match msg.payload {
            MessagePayload::Init => self.init(),
            MessagePayload::Block(block) => self.process_block(block),
            _ => panic!("Unknown message payload type"),
        }
    }
    fn node(&mut self) -> &mut Node {
        &mut self.node
    }
}

impl BlockProducer {
    pub fn new(own_id: NodeID) -> Self {
        Self{node: Node::new(own_id)}
    }

    pub fn init(&mut self) {}

    pub fn process_block(&mut self, block: Block) {
        println!("Processing block");
    }
}