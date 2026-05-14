use crate::types::ShardID;

#[derive(Clone)]
pub struct Config {
    pub num_shards: u32,
    pub num_block_producers: u32,
    pub num_chunk_producers_per_shard: u32,
    pub num_replicas_per_shard: u32,
    pub num_data_owners: u32,
    pub num_validators: u32,
    pub validator_sample_size: u32,
    pub num_chunk_data_parts: u32,
    pub availability_quorum: u32,
    pub endorsement_quorum: u32,
}

impl Config {
    pub fn default() -> Self {
        let num_shards = 2;
        Self {
            num_shards: 2,
            num_block_producers: 2,
            num_chunk_producers_per_shard: 2,
            num_replicas_per_shard: 2,
            num_data_owners: 8,
            num_validators: 4 * num_shards,
            validator_sample_size: 4,
            num_chunk_data_parts: 4,
            availability_quorum: 3, // This many out of chunk_data_parts are
            endorsement_quorum: 3, // This many out of validator_sample_size endorsements yield a StateCert.
        }
    }

    pub fn all_shards(&self) -> Vec<ShardID> {
        let mut shards = vec![];
        for shard in 0..self.num_shards {
            shards.push(ShardID(shard));
        }
        shards
    }
}
