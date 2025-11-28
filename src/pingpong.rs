use rand::Rng;
use crate::message::{Message, MessageHandler, MessagePayload};
use crate::node::{Node, NodeID};

pub struct PingPongNode {
    node: Node,
    counter: u32,
}

impl MessageHandler for PingPongNode {
    fn handle_message(&mut self, msg: Message) {
        match msg.payload {
            MessagePayload::Init => self.init(),
            MessagePayload::Ping(counter) => self.process_ping(msg.source, counter),
            MessagePayload::Pong(counter) => self.process_pong(msg.source, counter),
            _ => panic!("Unknown message payload type"),
        }
    }
    fn node(&mut self) -> &mut Node {
        &mut self.node
    }
}

impl PingPongNode {
    pub fn new(own_id: NodeID) -> Self {
        assert!(own_id == NodeID::from_str("0") || own_id == NodeID::from_str("1"));
        Self{node: Node::new(own_id), counter: 0}
    }

    fn other_node(&self) -> NodeID {
        if self.node.id() == NodeID::from_str("0") {
            NodeID::from_str("1")
        } else {
            NodeID::from_str("0")
        }
    }

    fn init(&mut self) {
        self.send_ping();
    }

    fn send_ping(&mut self) {
        self.node.send_message(MessagePayload::Ping(self.counter), self.other_node());
        self.counter += 1;
    }

    fn process_ping(&mut self, source: NodeID, counter: u32) {
        self.node.advance_time(rand::rng().random_range(10..1000));
        self.node.send_message(MessagePayload::Pong(counter), source)
    }

    fn process_pong(&mut self, _: NodeID, _: u32) {
        if self.counter < 10 {
            self.send_ping();
        }
    }
}
