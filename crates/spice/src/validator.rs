use sim_core::message::{Message, MessageHandler};
use sim_core::node::Node;

use crate::config::Config;
use crate::core_state::CoreState;
use crate::payload::MessagePayload;
use crate::types::{Block, ChunkID, StateWitness};

/// Receives state witnesses from replicas, "verifies" each one (a no-op for now), and
/// broadcasts a corresponding state endorsement to every block producer. Also tracks the
/// chain (currently unused, but needed once witness verification consults state).
pub struct Validator {
    core_state: CoreState,
}

impl MessageHandler<MessagePayload> for Validator {
    fn handle_message(&mut self, node: &mut Node<MessagePayload>, msg: Message<MessagePayload>) {
        match msg.payload {
            MessagePayload::Block(block) => self.process_block(block),
            MessagePayload::StateWitness(witness) => self.process_state_witness(node, witness),
            _ => panic!("Unknown message payload type"),
        }
    }
}

impl Validator {
    pub fn new(config: Config) -> Self {
        Self { core_state: CoreState::new(config) }
    }

    fn process_block(&mut self, block: Block) {
        self.core_state.apply_block(block);
    }

    fn process_state_witness(&mut self, node: &mut Node<MessagePayload>, witness: StateWitness) {
        self.submit_state_endorsement(node, witness.chunk_id);
    }

    // Send a state endorsement for `chunk_id` to every block producer.
    fn submit_state_endorsement(&mut self, node: &mut Node<MessagePayload>, chunk_id: ChunkID) {
        for block_producer in self.core_state.block_producer_ids() {
            node.send_message(MessagePayload::StatementStateEndorsement(chunk_id.clone()), block_producer);
        }
    }
}
