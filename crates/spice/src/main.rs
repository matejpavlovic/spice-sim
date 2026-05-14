use sim_core::simulator::Simulator;
use spice::block_producer::BlockProducer;
use spice::chunk_producer::ChunkProducer;
use spice::config::Config;
use spice::core_state::CoreState;
use spice::data_owner::DataOwner;
use spice::payload::MessagePayload;
use spice::replica::Replica;
use spice::types::ShardID;
use spice::validator::Validator;

const MAX_SIMULATION_TIME: u64 = 1000;

fn main() {
    let config = Config::default();
    let core_state = CoreState::new(config.clone());

    let mut sim: Simulator<MessagePayload> = Simulator::new();
    for node_id in core_state.block_producer_ids() {
        sim.register(node_id, BlockProducer::new(config.clone()));
    }
    for i in 0..config.num_shards {
        for node_id in core_state.chunk_producer_ids(ShardID(i)) {
            sim.register(node_id, ChunkProducer::new(ShardID(i), config.clone()));
        }
    }
    for node_id in core_state.data_owner_ids() {
        sim.register(node_id, DataOwner::new(config.clone()));
    }
    for i in 0..config.num_shards {
        for node_id in core_state.replica_ids(ShardID(i)) {
            sim.register(node_id, Replica::new(ShardID(i), config.clone()));
        }
    }
    for node_id in core_state.validator_ids() {
        sim.register(node_id, Validator::new(config.clone()));
    }

    sim.run(MAX_SIMULATION_TIME);
}
