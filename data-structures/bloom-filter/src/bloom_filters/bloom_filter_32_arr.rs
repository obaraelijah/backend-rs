use crate::bloom_filter::BloomFilter;

pub struct BloomFilter32 {
    bits: [bool; 32],
}

impl BloomFilter32 {
    fn additive_hasher(key: &str, seed: usize) -> usize {
        key.chars().fold(0, |acc, ch| -> usize {
            (acc + seed + (ch as usize % 32)) % 32 // modulo math return 0 - 31
        })
    }
}


impl BloomFilter for BloomFilter32 {
    fn insert(&mut self, key: &str) {
        let hash_a = Self::additive_hasher(key, 0);
        let hash_b = Self::additive_hasher(key, 1);

        self.bits[hash_a] = true;
        self.bits[hash_b] = true;
    }
    
    fn contains(&self, key: &str) -> bool {
        let hash_a = Self::additive_hasher(key, 0);
        let hash_b = Self::additive_hasher(key, 1);

        self.bits[hash_a] && self.bits[hash_b] 
    }
}