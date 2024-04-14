use crate::consistent_hash_ring::ConsitentHashRing;

#[derive(Clone)]
struct CHRVecNode<ConsumerInfo>
where
    ConsumerInfo: Clone,
{
    hash: u64,
    key: String,

    /// Data stored about the consumer like IP address, port, etc.
    data: ConsumerInfo,
}

/// Consistent Hash Ring implementation using vector
struct CHRVec<ConsumerInfo> 
where 
    ConsumerInfo: CLone,
{
    consumers: Vec<CHRVecNode<ConsumerInfo>>,
    virtual_nodes: usize // stores the number of virtual nodes used for each consumer and vn are used for load balancing
}

impl<ConsumerInfo> CHRVec<ConsumerInfo> 
where 
    ConsumerInfo: Clone,
{
    pub fn new(virtual_nodes_per_consumer: usize) -> Self {
        Self {
            consumers: Vec::new(),
            virtual_nodes: virtual_nodes_per_consumer,
        }
    }

    fn hash(key: &str) -> u64 {
        seahash::hash(key.as_bytes())
    }
}
