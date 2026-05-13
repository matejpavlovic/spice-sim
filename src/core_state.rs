use std::collections::HashSet;
use crate::types::{Block, ChunkID, Height, ShardID};
use crate::config::Config;
use crate::node::NodeID;

pub struct CoreState {
    config: Config,
    canonical_chain: Vec<Block>,
    avail_certs: HashSet<ChunkID>,
    state_certs: HashSet<ChunkID>,
}

impl CoreState {
    pub fn new(config: Config) -> Self {
        Self{
            config,
            canonical_chain: Vec::new(),
            avail_certs: HashSet::new(),
            state_certs: HashSet::new(),
        }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn block_producer_ids(&self) -> Vec<NodeID> {
        let mut ids = Vec::new();
        for i in 0..self.config.num_block_producers {
            ids.push(NodeID::from_str(format!("block-producer-{}", i).as_str()))
        }
        ids
    }

    pub fn chunk_producer_ids(&self, shard: ShardID) -> Vec<NodeID> {
        let mut ids = Vec::new();
        for i in 0..self.config.num_chunk_producers_per_shard {
            ids.push(NodeID::from_str(format!("chunk-producer-{}-{}", shard, i).as_str()))
        }
        ids
    }

    pub fn chunk_producer_ids_all_shards(&self) -> Vec<NodeID> {
        let mut ids = Vec::new();
        for shard in self.config.all_shards() {
            ids.append(&mut self.chunk_producer_ids(shard));
        }
        ids
    }

    pub fn replica_ids(&self, shard: ShardID) -> Vec<NodeID> {
        let mut ids = Vec::new();
        for i in 0..self.config.num_replicas_per_shard {
            ids.push(NodeID::from_str(format!("replica-{}-{}", shard, i).as_str()))
        }
        ids
    }

    pub fn replica_ids_all_shards(&self) -> Vec<NodeID> {
        let mut ids = Vec::new();
        for shard in self.config.all_shards() {
            ids.append(&mut self.replica_ids(shard));
        }
        ids
    }

    pub fn data_owner_ids(&self) -> Vec<NodeID> {
        let mut ids = Vec::new();
        for i in 0..self.config.num_data_owners {
            ids.push(NodeID::from_str(format!("data-owner-{}", i).as_str()))
        }
        ids
    }

    pub fn validator_ids(&self) -> Vec<NodeID> {
        let mut ids = Vec::new();
        for i in 0..self.config.num_validators {
            ids.push(NodeID::from_str(format!("validator-{}", i).as_str()))
        }
        ids
    }

    pub fn block_producer_at(&self, height: Height) -> NodeID {
        let index = u32::from(height) % self.config.num_block_producers;
        NodeID::from_str(format!("block-producer-{}", index).as_str())
    }

    pub(crate) fn chunk_producer_at(&self, height: Height, shard: ShardID) -> NodeID {
        let index = u32::from(height) % self.config.num_block_producers;
        NodeID::from_str(format!("chunk-producer-{}-{}", shard, index).as_str())
    }

    pub fn data_owner_at(&self, height: Height, shard: ShardID) -> Vec<NodeID> {
        Self::round_robin(height, shard, self.config.num_shards, self.config.num_chunk_data_parts, self.config.num_data_owners, "data-owner-")
    }

    pub fn validators_at(&self, height: Height, shard: ShardID) -> Vec<NodeID> {
        Self::round_robin(height, shard, self.config.num_shards, self.config.validator_sample_size, self.config.num_validators, "validator-")
    }

    pub fn apply_block(&mut self, block: Block) {
        if let Some(old_head) = self.canonical_chain.last() {
            if block.parent_hash != old_head.hash() {
                panic!("Block hash mismatch. Out-of-order blocks not implemented. {:?}", block);
            }
        } else if block != Block::genesis() {
            panic!("received non-genesis block on empty chain");
        }


        for cert in &block.avail_certs {
            self.avail_certs.insert(cert.chunk_id.clone());
        }

        for cert in &block.state_certs {
            self.state_certs.insert(cert.chunk_id.clone());
        }

        self.canonical_chain.push(block);
    }

    fn round_robin(
        height: Height,
        shard: ShardID,
        num_shards: u32,
        num_items: u32,
        pool_size: u32,
        prefix: &str,
    ) -> Vec<NodeID> {
        //          < ---------- items in previous and this block ----------- >
        //           < -- batches in previous and this block --- >
        //                    < -- batches in previous blocks -- >
        let mut index = ((shard + height /*   */ * /*   */ num_shards) * num_items) % pool_size;
        let mut node_ids = vec![];
        for _ in 0..num_items {
            node_ids.push(NodeID::from_str(format!("{}{}", prefix, index).as_str()));
            index = (index + 1) % pool_size
        }
        node_ids
    }
}