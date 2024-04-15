use std::isize;

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

    fn extend_consumers(
        &mut self,
        nodes_to_insert_iter: impl Iterator<Item = CHRVecNode<ConsumerInfo>> + Clone,
    ) {
        let prev_len = self.consumers.len();
        self.consumers.reserve(self.virtual_nodes);

        self.consumers.extend(nodes_to_insert_iter.clone());

        let mut nodes_to_insert = nodes_to_insert_iter.collect::<Vec<_>>();
        nodes_to_insert.sort_by_key(|node| node.hash);

        let mut nodes_to_insert_index = (self.virtual_nodes - 1) as isize;
        let mut consumers_index: isize = prev_len as isize - 1;

        for back_index in (0..self.consumers.len()).rev() {
            if consumers_index < 0
                || (nodes_to_insert_index >= 0
                    && nodes_to_insert[nodes_to_insert_index as usize].hash
                        > self.consumers[consumers_index as usize].hash)
            {
                self.consumers[back_index] = nodes_to_insert.pop().unwrap();
                nodes_to_insert_index -= 1;
            } else {
                if consumers_index as usize == back_index {
                    // already inserted now order won't change from here on
                    break;
                }

                self.consumers.swap(consumers_index as usize, back_index);
                consumers_index -= 1;
            }
        }
    }
}

impl<ConsumerInfo> ConsitentHashRing for CHRVec<ConsumerInfo>
where
    ConsumerInfo: CLone,
{
    type ConsumerInfo = ConsumerInfo;

    fn add_consumer(&mut self, key: &str, data: Self::ConsumerInfo) {
        let nodes_to_insert_iter = (0..self.virtual_nodes).map(|i| {
            let key = format!("{}_{}", key, i);
            let hash = Self::hash(&key);
            CHRVecNode {
                key,
                hash,
                data: data.clone(),
            }
        });

        // self.consumers.reserve(self.virtual_nodes);
        // self.consumers.extend(nodes_to_insert_iter);
        // self.consumers.sort_by_key(|consumer| consumer.hash);
        self.extend_consumers(nodes_to_insert_iter);
    }

    fn remove_consumer(&mut self, key: &str) {
        self.consumers
            .retain(|consumer| !consumer.key.starts_with(key));
    }

    
    fn get_consumer(&self, key: &str) -> Option<&Self::ConsumerInfo> {
        let hash = Self::hash(key);

        if self.consumers.is_empty() {
            return None;
        }

        let index_result = self
            .consumers
            .binary_search_by_key(&hash, |consumer| consumer.hash);

        let index = match index_result {
            Ok(index) => index,
            Err(index) => {
                if index >= self.consumers.len() {
                    0
                } else {
                    index
                }
            }
        };

        Some(&self.consumers[index].data)
    }
}